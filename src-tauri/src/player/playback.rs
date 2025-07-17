use crate::player::playback_driver::{AudioCommand, PlaybackDriver};
use crate::player::{queue::Queue, track::Track};

use anyhow::{anyhow, Error, Result};
use serde::Serialize;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

pub enum PlaybackEvent {
    HistoryUpdate,
    FailedOpeningFile(Error),
    TrackCompleted,
    Shutdown,
    Progress(f64, u64), // percent completed and frames played
    TrackChanged(Option<Track>),
    QueueChanged(Vec<Track>),
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum PlaybackState {
    Playing,
    Paused,
    Stopped,
}

pub struct Playback {
    driver: Box<dyn PlaybackDriver>,
    pub state: PlaybackState,
    current_track: Option<Track>,
    queue: Option<Queue>,
    pub history: Vec<Track>,
    event_sender: mpsc::Sender<PlaybackEvent>,
    progress: f64,
    current_track_added_to_history: bool,
}

impl Playback {
    pub fn create(
        driver: Box<dyn PlaybackDriver>,
        on_progress_update: impl Fn(f64, u64) + Send + 'static,
        on_history_update: impl Fn(&Vec<Track>, Option<&Track>) + Send + 'static,
        on_track_changed: impl Fn(Option<&Track>) + Send + 'static,
        on_queue_changed: impl Fn(&Vec<Track>) + Send + 'static,
    ) -> Arc<Mutex<Self>> {
        let (event_sender, event_receiver) = mpsc::channel();

        let playback = Arc::new(Mutex::new(Self {
            driver,
            queue: None,
            history: Vec::new(),
            current_track: None,
            state: PlaybackState::Stopped,
            event_sender,
            progress: 0.0,
            current_track_added_to_history: false,
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
                    PlaybackEvent::HistoryUpdate => {
                        if let Ok(playback) = playback_clone.lock() {
                            on_history_update(&playback.history, playback.current_track.as_ref());
                        }
                    }
                    PlaybackEvent::TrackChanged(track) => {
                        on_track_changed(track.as_ref());
                    }
                    PlaybackEvent::QueueChanged(queue) => {
                        on_queue_changed(&queue);
                    }
                    PlaybackEvent::Progress(percent, frames_played) => {
                        if let Ok(mut playback) = playback_clone.lock() {
                            if playback.state == PlaybackState::Playing {
                                if percent > 2.0 && !playback.current_track_added_to_history {
                                    if let Some(track) = playback.current_track.as_ref().cloned() {
                                        playback.history.push(track);
                                        playback.current_track_added_to_history = true;
                                        playback
                                            .event_sender
                                            .send(PlaybackEvent::HistoryUpdate)
                                            .ok();
                                    }
                                }
                                playback.progress = percent;
                                on_progress_update(percent, frames_played);
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
        self.enqueue_internal(track);
        self.event_sender.send(PlaybackEvent::QueueChanged(self.get_queue())).ok();
    }

    fn enqueue_internal(&mut self, track: Track) {
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
            self.enqueue_internal(track);
        }
        self.event_sender.send(PlaybackEvent::QueueChanged(self.get_queue())).ok();
    }

    pub fn clear_queue(&mut self) {
        if let Some(queue) = &mut self.queue {
            queue.clear();
        }
        self.event_sender.send(PlaybackEvent::QueueChanged(self.get_queue())).ok();
    }

    pub fn play(&mut self) -> Result<PlaybackState> {
        if self.current_track.is_some() && self.state == PlaybackState::Paused {
            return self.resume_play();
        }

        let track_changed = self.current_track.is_none();
        if self.current_track.is_none() {
            if let Some(queue) = &mut self.queue {
                self.current_track = queue.dequeue();
                if track_changed {
                    self.event_sender.send(PlaybackEvent::TrackChanged(self.current_track.clone())).ok();
                    self.event_sender.send(PlaybackEvent::QueueChanged(self.get_queue())).ok();
                }
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
        self.stop()?;
        self.current_track_added_to_history = false;
        let result = self.play(); // will take next in queue and emit events
        result
    }

    pub fn previous(&mut self) -> Result<PlaybackState> {
        self.state = PlaybackState::Stopped;
        self.current_track_added_to_history = false;
        self.driver.send_command(AudioCommand::Pause)?;
        let mut history_changed = false;
        while let last = self.history.pop() {
            history_changed = true;
            if last.is_none() {
                break;
            }
            if last != self.current_track {
                self.current_track = last.clone();
                break;
            }
        }
        if history_changed {
            self.event_sender.send(PlaybackEvent::HistoryUpdate)?;
        }
        let result = self.play();
        self.event_sender.send(PlaybackEvent::TrackChanged(self.current_track.clone())).ok();
        result
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
            .send_command(AudioCommand::Pause)
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

    pub fn get_queue(&self) -> Vec<Track> {
        self.queue.as_ref().map(|q| q.tracks()).unwrap_or_else(Vec::new)
    }

    pub fn get_current_track(&self) -> Option<Track> {
        self.current_track.clone()
    }
}

impl Drop for Playback {
    fn drop(&mut self) {
        println!("Playback is being dropped, sending shutdown event");
        self.event_sender
            .send(PlaybackEvent::Shutdown)
            .expect("Failed to send shutdown event");
    }
}

#[cfg(test)]
#[path = "./playback.tests.rs"]
mod tests;
