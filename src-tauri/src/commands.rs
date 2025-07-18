use std::time::Duration;

use crate::player::{playback::PlaybackState, track::Track};
use anyhow::Error;
use serde::{Deserialize, Serialize};
use tauri::{ipc::Channel, State};

use crate::{AppState, ProgressEvent};

#[derive(Deserialize, Debug)]
pub struct ControlPlaybackPayload {
    pub command: String,
    pub volume: Option<f32>,
    pub seek_position: Option<u64>, // in milliseconds
}

#[derive(Debug, Clone, Serialize)]
pub struct PlaybackError(String);

impl PlaybackError {
    fn from(err: Error) -> Self {
        PlaybackError(err.to_string())
    }
}

#[tauri::command]
pub fn subscribe_to_progress(state: State<'_, AppState>, on_progress: Channel<ProgressEvent>) {
    let mut channel_guard = state.progress_channel.lock().unwrap();
    *channel_guard = Some(on_progress);
}

#[tauri::command]
pub fn control_playback(
    state: State<'_, AppState>,
    payload: ControlPlaybackPayload,
) -> Result<PlaybackState, PlaybackError> {
    let mut playback = state.playback.lock().unwrap();

    match payload.command.as_str() {
        "Play" => playback.play().map_err(PlaybackError::from),
        "Pause" => playback.pause().map_err(PlaybackError::from),
        "Next" => playback.next().map_err(PlaybackError::from),
        "Seek" => playback
            .seek(Duration::from_millis(payload.seek_position.unwrap_or(0)))
            .map_err(PlaybackError::from),
        "Previous" => playback.previous().map_err(PlaybackError::from),
        "SetVolume" => {
            if let Some(volume) = payload.volume {
                playback.set_volume(volume).map_err(PlaybackError::from)
            } else {
                Err(PlaybackError("Invalid payload".to_string()))
            }
        }
        _ => Err(PlaybackError("Unknown action".to_string())),
    }
}

#[tauri::command]
pub fn get_library_path(state: State<'_, AppState>) -> Result<String, String> {
    let config = state.config.lock().map_err(|e| e.to_string())?;
    Ok(config.library_path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn set_library_path(
    state: State<'_, AppState>,
    app_handle: tauri::AppHandle,
    path: String,
) -> Result<(), String> {
    let mut config = state.config.lock().map_err(|e| e.to_string())?;
    let new_path = std::path::PathBuf::from(&path);

    config
        .update_library_path(new_path.clone())
        .map_err(|e| e.to_string())?;
    config.save(&app_handle).map_err(|e| e.to_string())?;

    // Update library path and rescan
    let mut library = state.library.lock().map_err(|e| e.to_string())?;
    library.update(Some(new_path), None);
    library.rescan();

    // Update playback queue with new tracks
    let mut playback = state.playback.lock().map_err(|e| e.to_string())?;
    playback.clear_queue();
    playback.enqueue_multiple(library.get_tracks());

    Ok(())
}

#[tauri::command]
pub fn rescan_library(state: State<'_, AppState>) -> Result<(), String> {
    let mut library = state.library.lock().map_err(|e| e.to_string())?;
    library.rescan();

    // Update playback queue with rescanned tracks
    let mut playback = state.playback.lock().map_err(|e| e.to_string())?;
    playback.clear_queue();
    playback.enqueue_multiple(library.get_tracks());

    Ok(())
}

#[tauri::command]
pub fn get_library_tracks(state: State<'_, AppState>) -> Result<Vec<Track>, String> {
    let library = state.library.lock().map_err(|e| e.to_string())?;
    Ok(library.get_tracks())
}

#[tauri::command]
pub fn select_track_from_queue(
    state: State<'_, AppState>,
    track_id: String,
) -> Result<PlaybackState, PlaybackError> {
    let mut playback = state.playback.lock().unwrap();
    playback
        .select_track_from_queue(&track_id)
        .map_err(PlaybackError::from)
}

#[tauri::command]
pub fn play_from_library(
    state: State<'_, AppState>,
    track_id: String,
) -> Result<PlaybackState, PlaybackError> {
    let mut playback = state.playback.lock().unwrap();
    let track = state
        .library
        .lock()
        .unwrap()
        .get_track_by_id(&track_id)
        .ok_or_else(|| PlaybackError("Track not found".to_string()))?;
    playback.prepend(track);
    if playback.state == PlaybackState::Playing || playback.state == PlaybackState::Paused {
        playback.next().map_err(PlaybackError::from)
    } else {
        playback.play().map_err(PlaybackError::from)
    }
}
