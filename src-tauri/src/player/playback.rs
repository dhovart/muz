use crate::player::playback_driver::{AudioCommand, PlaybackDriver};
use crate::player::{queue::Queue, track::Track};

use anyhow::{anyhow, Result};
use std::sync::mpsc::{self, Sender};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::sync::Mutex;

pub enum PlaybackState {
    Playing(Duration),
    Paused(Duration),
    Stopped,
}

pub struct Playback {
    driver: Box<dyn PlaybackDriver>,
    state: PlaybackState,
    current_track: Option<Track>,
    pub queue: Option<Queue>,
    history: Vec<Track>,
}

impl Playback {
    pub fn new(driver: Box<dyn PlaybackDriver>) -> Self {
        Self {
            driver,
            queue: None,
            history: Vec::new(),
            current_track: None,
            state: PlaybackState::Stopped,
        }
    }

    pub fn create_shared(driver: Box<dyn PlaybackDriver>) -> Arc<Mutex<Playback>> {
        Arc::new(Mutex::new(Self::new(driver)))
    }

    pub fn current_track(&self) -> Option<&Track> {
        match self.state {
            PlaybackState::Playing(_) | PlaybackState::Paused(_) => self.current_track.as_ref(),
            PlaybackState::Stopped => None,
        }
    }

    pub fn play(&mut self, track: Track) -> Result<()> {
        self.current_track = if let Some(queue) = &mut self.queue {
            let pos: Option<usize> = queue.iter().position(|t| *t == track);
            println!("Position in queue: {pos:?}");
            if let Some(pos) = pos {
                println!("Track already in queue at position: {pos}, moving item");
                queue.move_item(pos, 0)
            }

            queue.dequeue()
        } else {
            Some(track)
        };

        let track_path = self.current_track.clone().unwrap().path;
        println!("Loading file: {}", track_path.display());
        self.driver
            .send_command(AudioCommand::LoadFile(track_path))?;
        println!("Playing file");
        self.state = PlaybackState::Playing(Duration::from_secs(0));
        self.history.push(self.current_track.clone().unwrap());
        //let command_sender = self.driver.get_command_sender();
        self.driver.send_command(AudioCommand::Play(Box::new(|| {
            println!("Playback completed, moving to next track");
            // find some way to call self.next() or send a message to do so
        })))?;

        Ok(())
    }

    pub fn resume_play(&mut self) -> Result<()> {
        match &self.state {
            PlaybackState::Paused(duration) => {
                self.state = PlaybackState::Playing(*duration);
                self.driver
                    .send_command(AudioCommand::Resume)
                    .map_err(|e| anyhow!("Failed to resume playback: {e}"))
            }
            _ => Err(anyhow!("No track to resume playback")),
        }
    }

    pub fn pause(&mut self) -> Result<()> {
        if let PlaybackState::Playing(duration) = self.state {
            self.state = PlaybackState::Paused(duration);
            self.driver
                .send_command(AudioCommand::Pause)
                .map_err(|e| anyhow!("Failed to pause playback: {e}"))
        } else {
            Err(anyhow!("No track is currently playing"))
        }
    }

    pub fn stop(&mut self) -> Result<()> {
        self.state = PlaybackState::Stopped;
        self.current_track = None;
        self.driver
            .send_command(AudioCommand::Stop)
            .map_err(|e| anyhow!("Failed to stop playback: {e}"))
    }

    pub fn seek(&mut self, offset: Duration) -> Result<()> {
        todo!("Implement seek functionality");
    }

    pub fn next(&mut self) -> Result<()> {
        if let Some(queue) = self.queue.as_mut() {
            if queue.is_empty() {
                Err(anyhow!("Queue is empty"))
            } else if let Some(next_track) = queue.dequeue() {
                self.play(next_track)
            } else {
                Err(anyhow!("End of queue"))
            }
        } else {
            Err(anyhow!("No queue available"))
        }
    }

    pub fn previous(&mut self) -> Result<()> {
        if let Some(track) = self.history.pop() {
            self.play(track)
        } else {
            Err(anyhow!("No previous track available"))
        }
    }

    pub fn set_volume(&mut self, volume: f32) -> Result<()> {
        self.driver
            .send_command(AudioCommand::SetVolume(volume.clamp(0.0, 1.0)))
            .map_err(|e| anyhow!("Failed to set volume: {e}"))
    }
}

#[cfg(test)]
#[path = "./playback.tests.rs"]
mod tests;
