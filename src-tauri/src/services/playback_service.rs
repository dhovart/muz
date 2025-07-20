use crate::commands::ControlPlaybackPayload;
use crate::player::{playback::Playback, playback::PlaybackState, track::Track};
use anyhow::Result;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct PlaybackService {
    playback: Arc<Mutex<Playback>>,
}

impl PlaybackService {
    pub fn new(playback: Arc<Mutex<Playback>>) -> Self {
        Self { playback }
    }

    pub fn control_playback(&self, payload: ControlPlaybackPayload) -> Result<PlaybackState> {
        let mut playback = self.playback.lock().unwrap();

        match payload.command.as_str() {
            "Play" => playback.play(),
            "Pause" => playback.pause(),
            "Next" => playback.next(),
            "Previous" => playback.previous(),
            "Seek" => {
                println!("Seek payload: {:?}", payload);
                let position = payload.seek_position.unwrap_or(0);
                println!("Seek position received: {} ms", position);
                playback.seek(Duration::from_millis(position))
            }
            "SetVolume" => {
                if let Some(vol) = payload.volume {
                    playback.set_volume(vol)
                } else {
                    Err(anyhow::anyhow!("Invalid volume payload"))
                }
            }
            _ => Err(anyhow::anyhow!("Unknown playback command")),
        }
    }

    pub fn play_single_track(&self, track: Track) -> Result<PlaybackState> {
        let mut playback = self.playback.lock().unwrap();
        
        playback.prepend(track);
        if playback.state == PlaybackState::Playing || playback.state == PlaybackState::Paused {
            playback.next()
        } else {
            playback.play()
        }
    }

    pub fn play_album_tracks(&self, album_tracks: Vec<Track>, track_id: &str) -> Result<PlaybackState> {
        let mut playback = self.playback.lock().unwrap();

        let selected_track_id = album_tracks
            .iter()
            .find(|track| track.id == track_id)
            .map(|track| track.id.clone())
            .ok_or_else(|| anyhow::anyhow!("Track not found in album"))?;

        playback.clear_queue();
        playback.enqueue_multiple(album_tracks);

        playback.select_track_from_queue(&selected_track_id)
    }

    pub fn select_from_queue(&self, track_id: &str) -> Result<PlaybackState> {
        let mut playback = self.playback.lock().unwrap();
        playback.select_track_from_queue(track_id)
    }

    pub fn clear_queue_and_enqueue(&self, tracks: Vec<Track>) -> Result<()> {
        let mut playback = self.playback.lock().unwrap();
        playback.clear_queue();
        playback.enqueue_multiple(tracks);
        Ok(())
    }

    pub fn set_spectrum_computation(&self, should_compute: bool) -> Result<()> {
        let mut playback = self.playback.lock().unwrap();
        playback.set_spectrum_computation(should_compute)
    }
}
