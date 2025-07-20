pub mod awedio;
pub mod factory;
pub mod rodio;

use crate::player::{playback::PlaybackEvent, track::Track};
use anyhow::Result;
use std::sync::mpsc::Sender;
use std::time::Duration;

pub trait PlaybackDriver: Send {
    /// Start playing a track with progress events sent to the given channel
    fn play(&mut self, track: Track, progress_sender: Sender<PlaybackEvent>) -> Result<()>;

    /// Pause current playback
    fn pause(&mut self) -> Result<()>;

    /// Resume current playback
    fn resume(&mut self) -> Result<()>;

    /// Clear/stop current playback
    fn clear(&mut self) -> Result<()>;

    /// Set playback volume (0.0 to 1.0)
    fn set_volume(&mut self, volume: f32) -> Result<()>;

    /// Seek to a specific position in the current track
    fn seek(&mut self, position: Duration) -> Result<()>;

    /// Control whether spectrum computation should be performed
    fn set_spectrum_computation(&mut self, should_compute: bool) -> Result<()>;
}
