use crate::player::playback;
use crate::player::playback_driver::{AudioCommand, PlaybackDriver};
use crate::player::{queue::Queue, track::Track};

use anyhow::{anyhow, Result};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

pub enum PlaybackEvent {
    TrackCompleted,
    Shutdown,
}

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
    event_sender: mpsc::Sender<PlaybackEvent>,
}

impl Playback {
    pub fn create(driver: Box<dyn PlaybackDriver>) -> Arc<Mutex<Self>> {
        let (event_sender, event_receiver) = mpsc::channel();

        let playback = Arc::new(Mutex::new(Self {
            driver,
            queue: None,
            history: Vec::new(),
            current_track: None,
            state: PlaybackState::Stopped,
            event_sender,
        }));

        let playback_clone = Arc::clone(&playback);

        // Spawn the event handling thread
        thread::spawn(move || {
            for event in event_receiver {
                match event {
                    PlaybackEvent::TrackCompleted => {
                        if let Ok(mut playback) = playback_clone.lock() {
                            println!("Track completed, moving to next track");
                            playback
                                .play(None)
                                .unwrap_or_else(|e| println!("Error moving to next track: {e}"));
                        }
                    }
                    PlaybackEvent::Shutdown => break,
                }
            }
            println!("Playback event loop shutting down");
        });

        playback
    }

    pub fn current_track(&self) -> Option<&Track> {
        match self.state {
            PlaybackState::Playing(_) | PlaybackState::Paused(_) => self.current_track.as_ref(),
            PlaybackState::Stopped => None,
        }
    }

    pub fn enqueue(&mut self, track: Track) {
        println!("Enqueuing track: {track:?}");
        if let Some(queue) = &mut self.queue {
            queue.enqueue(track);
        } else {
            println!("Creating queue");
            let mut queue = Queue::new();
            queue.enqueue(track);
            self.queue = Some(queue);
        }
    }

    pub fn play(&mut self, track: Option<Track>) -> Result<()> {
        self.current_track = if let Some(queue) = &mut self.queue {
            if let Some(track) = track {
                let pos: Option<usize> = queue.iter().position(|t| *t == track);
                println!("Position in queue: {pos:?}");
                if let Some(pos) = pos {
                    println!("Track already in queue at position: {pos}, moving item");
                    queue.move_item(pos, 0)
                }
            }

            queue.dequeue()
        } else {
            track
        };

        if self.current_track.is_none() {
            return Err(anyhow!("No track to play"));
        }

        let track_path = self.current_track.clone().unwrap().path;
        let (completion_sender, completion_receiver) = mpsc::channel();
        self.driver
            .send_command(AudioCommand::Play(track_path, completion_sender))?;
        self.state = PlaybackState::Playing(Duration::from_secs(0));
        self.history.push(self.current_track.clone().unwrap());

        let event_sender = self.event_sender.clone();

        thread::spawn(move || {
            if let Err(e) = completion_receiver.recv() {
                println!("Error receiving playback completion: {e}");
            } else {
                println!("Playback completed for file");
                event_sender
                    .send(PlaybackEvent::TrackCompleted)
                    .expect("Failed to send track completed event");
            }
        });

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

    pub fn previous(&mut self) -> Result<()> {
        if let Some(track) = self.history.pop() {
            self.play(Some(track))
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
