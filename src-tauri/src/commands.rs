use std::collections::HashMap;

use crate::player::{playback::PlaybackState, track::Track};
use crate::services::library_service::LibraryService;
use crate::services::playback_service::PlaybackService;
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
    state
        .playback_service
        .control_playback(payload)
        .map_err(PlaybackError::from)
}

#[tauri::command]
pub fn get_library_path(state: State<'_, AppState>) -> Result<String, String> {
    state
        .library_service
        .get_library_path()
        .map_err(|e| e.to_string())
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

    state
        .library_service
        .set_library_path(new_path)
        .map_err(|e| e.to_string())?;

    let tracks = state
        .library_service
        .get_library_tracks()
        .map_err(|e| e.to_string())?
        .into_values()
        .flatten()
        .collect();

    state
        .playback_service
        .clear_queue_and_enqueue(tracks)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn rescan_library(state: State<'_, AppState>) -> Result<(), String> {
    state
        .library_service
        .rescan_library()
        .map_err(|e| e.to_string())?;

    // Update playback queue with rescanned tracks
    let tracks = state
        .library_service
        .get_library_tracks()
        .map_err(|e| e.to_string())?
        .into_values()
        .flatten()
        .collect();

    state
        .playback_service
        .clear_queue_and_enqueue(tracks)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn get_library_tracks(
    state: State<'_, AppState>,
) -> Result<HashMap<String, Vec<Track>>, String> {
    state
        .library_service
        .get_library_tracks()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn select_track_from_queue(
    state: State<'_, AppState>,
    track_id: String,
) -> Result<PlaybackState, PlaybackError> {
    state
        .playback_service
        .select_from_queue(&track_id)
        .map_err(PlaybackError::from)
}

#[tauri::command]
pub fn play_from_library(
    state: State<'_, AppState>,
    track_id: String,
    album: Option<String>,
    artist: Option<String>,
) -> Result<PlaybackState, PlaybackError> {
    // If album and artist are provided, play the entire album
    if let (Some(album_name), Some(artist_name)) = (album, artist) {
        // Get album tracks using LibraryService
        let album_tracks = state
            .library_service
            .get_tracks_by_album(&album_name, &artist_name)
            .map_err(PlaybackError::from)?;

        // Use playback service to play the album
        state
            .playback_service
            .play_album_tracks(album_tracks, &track_id)
            .map_err(PlaybackError::from)
    } else {
        // Original behavior: just play the single track using LibraryService
        let track = state
            .library_service
            .get_track_by_id(&track_id)
            .map_err(PlaybackError::from)?;
        state
            .playback_service
            .play_single_track(track)
            .map_err(PlaybackError::from)
    }
}
