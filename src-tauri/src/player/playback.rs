use crate::player::playback_driver::{AudioCommand, PlaybackDriver};
use crate::player::{queue::Queue, track::Track};

use anyhow::{anyhow, Error, Result};
use serde::Serialize;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

pub enum PlaybackEvent {
    FailedOpeningFile(Error),
    TrackCompleted,
    Shutdown,
    Progress(f64),
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum PlaybackState {
    Playing,
    Paused,
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
        on_progress_update: impl Fn(f64) + Send + 'static,
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
                            playback
                                .driver
                                .send_command(AudioCommand::Clear)
                                .unwrap_or_else(|e| {
                                    eprintln!("Error clearing playback: {e}");
                                });
                            if let Some(current_track) = playback.current_track.take() {
                                println!("Appending {current_track:?} to history");
                                playback.history.push(current_track);
                            }

                            println!("Playing next track");
                            playback
                                .next()
                                .map_err(|e| {
                                    eprintln!("Error moving to next track: {e}");
                                    e
                                })
                                .ok();
                        }
                    }
                    PlaybackEvent::FailedOpeningFile(err) => {
                        println!("Failed to open file: {err}");
                    }
                    PlaybackEvent::Progress(percent) => {
                        if let Ok(playback) = playback_clone.lock() {
                            if matches!(playback.state, PlaybackState::Playing) {
                                on_progress_update(percent);
                            }
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
            PlaybackState::Playing | PlaybackState::Paused => self.current_track.as_ref(),
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

    pub fn play(&mut self) -> Result<PlaybackState> {
        if self.current_track.is_some() && matches!(self.state, PlaybackState::Paused) {
            return self.resume_play();
        }

        if self.current_track.is_none() {
            if let Some(queue) = &mut self.queue {
                self.current_track = queue.dequeue()
            };
        }

        self.driver.send_command(AudioCommand::Play(
            self.current_track
                .clone()
                .ok_or_else(|| anyhow!("No track to play"))?,
            self.event_sender.clone(),
        ))?;
        self.state = PlaybackState::Playing;

        Ok(self.state.clone())
    }

    pub fn next(&mut self) -> Result<PlaybackState> {
        self.current_track = None;
        self.play()
    }

    pub fn previous(&mut self) -> Result<PlaybackState> {
        if let Some(track) = self.history.pop() {
            self.driver.send_command(AudioCommand::Pause)?; // FIXME replace by Stop
            self.current_track = Some(track);
            self.play()
        } else {
            Err(anyhow!("No previous track available"))
        }
    }

    pub fn resume_play(&mut self) -> Result<PlaybackState> {
        match &self.state {
            PlaybackState::Paused => {
                self.state = PlaybackState::Playing;
                self.driver
                    .send_command(AudioCommand::Resume)
                    .map_err(|e| anyhow!("Failed to resume playback: {e}"))?;
                Ok(self.state.clone())
            }
            _ => Err(anyhow!("No track to resume playback")),
        }
    }

    pub fn pause(&mut self) -> Result<PlaybackState> {
        if let PlaybackState::Playing = self.state {
            self.state = PlaybackState::Paused;
            self.driver
                .send_command(AudioCommand::Pause)
                .map_err(|e| anyhow!("Failed to pause playback: {e}"))?;
        }
        Ok(self.state.clone())
    }

    pub fn stop(&mut self) -> Result<PlaybackState> {
        self.state = PlaybackState::Stopped;
        self.current_track = None;
        self.driver
            .send_command(AudioCommand::Clear)
            .map_err(|e| anyhow!("Failed to stop playback: {e}"))?;
        Ok(self.state.clone())
    }

    pub fn seek(&mut self, position: Duration) -> Result<PlaybackState> {
        self.driver
            .send_command(AudioCommand::Seek(position))
            .map_err(|e| anyhow!("Failed to seek: {e}"))?;
        Ok(self.state.clone())
    }

    pub fn set_volume(&mut self, volume: f32) -> Result<PlaybackState> {
        self.driver
            .send_command(AudioCommand::SetVolume(volume.clamp(0.0, 1.0)))
            .map_err(|e| anyhow!("Failed to set volume: {e}"))?;
        Ok(self.state.clone())
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
