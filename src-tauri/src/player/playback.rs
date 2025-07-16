use crate::player::playback_driver::{AudioCommand, PlaybackDriver};
use crate::player::{queue::Queue, track::Track};

use anyhow::{anyhow, Error, Result};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

pub enum PlaybackEvent {
    FailedOpeningFile(Error),
    TrackCompleted,
    Shutdown,
    Progress(u64),
}

#[derive(Debug, PartialEq)]
pub enum PlaybackState {
    Playing(Duration),
    Paused(Duration),
    Stopped,
}

pub struct Playback {
    driver: Box<dyn PlaybackDriver>,
    state: PlaybackState,
    current_track: Option<Track>,
    queue: Option<Queue>,
    history: Vec<Track>,
    event_sender: mpsc::Sender<PlaybackEvent>,
}

impl Playback {
    pub fn create(
        driver: Box<dyn PlaybackDriver>,
        on_progress_update: impl Fn(u64) + Send + 'static,
    ) -> Arc<Mutex<Self>> {
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

        thread::spawn(move || {
            for event in event_receiver {
                match event {
                    PlaybackEvent::TrackCompleted => {
                        println!("Track completed event received");
                        if let Ok(mut playback) = playback_clone.lock() {
                            if let Some(current_track) = playback.current_track.take() {
                                println!("Appending {current_track:?} to history");
                                playback.history.push(current_track);
                            }

                            println!("Playing next track");
                            playback
                                .next()
                                .unwrap_or_else(|e| println!("Error moving to next track: {e}"));
                        }
                    }
                    PlaybackEvent::FailedOpeningFile(err) => {
                        println!("Failed to open file: {err}");
                    }
                    PlaybackEvent::Progress(percent) => {
                        on_progress_update(percent);
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

    pub fn enqueue_multiple(&mut self, tracks: Vec<Track>) {
        for track in tracks {
            self.enqueue(track);
        }
    }

    pub fn play(&mut self) -> Result<()> {
        if let PlaybackState::Paused(_) = self.state {
            println!("Resuming playback");
            return self.resume_play();
        }

        if self.current_track.is_none() {
            if let Some(queue) = &mut self.queue {
                self.current_track = queue.dequeue()
            };
        }

        self.driver.send_command(AudioCommand::Clear)?;
        self.driver.send_command(AudioCommand::Play(
            self.current_track
                .clone()
                .ok_or_else(|| anyhow!("No track to play"))?,
            self.event_sender.clone(),
        ))?;
        self.state = PlaybackState::Playing(Duration::from_secs(0));

        Ok(())
    }

    pub fn next(&mut self) -> Result<()> {
        self.current_track = None;
        self.play()
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

    pub fn seek(&mut self, position: Duration) -> Result<()> {
        self.driver
            .send_command(AudioCommand::Seek(position))
            .map_err(|e| anyhow!("Failed to seek: {e}"))
    }

    pub fn previous(&mut self) -> Result<()> {
        if let Some(track) = self.history.pop() {
            self.current_track = Some(track);
            self.play()
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

impl Drop for Playback {
    fn drop(&mut self) {
        self.event_sender
            .send(PlaybackEvent::Shutdown)
            .expect("Failed to send shutdown event");
    }
}

#[cfg(test)]
#[path = "./playback.tests.rs"]
mod tests;
