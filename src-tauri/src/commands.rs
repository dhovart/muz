use std::collections::HashMap;

use crate::player::{playback::PlaybackState, track::Track};
use anyhow::Error;
use serde::{Deserialize, Serialize};
use tauri::{ipc::Channel, State};

use crate::{AppState, ProgressEvent, SpectrumEvent};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
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
pub async fn subscribe_to_progress(
    state: State<'_, AppState>,
    on_progress: Channel<ProgressEvent>,
) -> Result<(), String> {
    let mut channel_guard = state.progress_channel.lock().await;
    *channel_guard = Some(on_progress);
    Ok(())
}

#[tauri::command]
pub async fn subscribe_to_spectrum(
    state: State<'_, AppState>,
    on_spectrum: Channel<SpectrumEvent>,
) -> Result<(), String> {
    let mut channel_guard = state.spectrum_channel.lock().await;
    *channel_guard = Some(on_spectrum);
    Ok(())
}

#[tauri::command]
pub async fn unsubscribe_from_progress(state: State<'_, AppState>) -> Result<(), String> {
    let mut channel_guard = state.progress_channel.lock().await;
    *channel_guard = None;
    Ok(())
}

#[tauri::command]
pub async fn unsubscribe_from_spectrum(state: State<'_, AppState>) -> Result<(), String> {
    let mut channel_guard = state.spectrum_channel.lock().await;
    *channel_guard = None;
    Ok(())
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
pub async fn get_library_path(state: State<'_, AppState>) -> Result<String, String> {
    state
        .library_service
        .library_path()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_library_path(
    state: State<'_, AppState>,
    app_handle: tauri::AppHandle,
    path: String,
) -> Result<(), String> {
    let new_path = std::path::PathBuf::from(&path);

    let mut config = state.config.lock().await;
    config
        .update_library_path(new_path.clone())
        .map_err(|e| e.to_string())?;
    config.save(&app_handle).await.map_err(|e| e.to_string())?;
    drop(config);

    state
        .library_service
        .set_library_path(new_path)
        .await
        .map_err(|e| e.to_string())?;

    let tracks = state
        .library_service
        .library_tracks()
        .await
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
pub async fn rescan_library(state: State<'_, AppState>) -> Result<(), String> {
    state
        .library_service
        .rescan_library()
        .await
        .map_err(|e| e.to_string())?;

    // Update playback queue with rescanned tracks
    let tracks = state
        .library_service
        .library_tracks()
        .await
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
pub async fn get_albums_by_artist(
    state: State<'_, AppState>,
) -> Result<HashMap<String, HashMap<String, Vec<Track>>>, String> {
    state
        .library_service
        .albums_by_artist()
        .await
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
pub async fn play_from_library(
    state: State<'_, AppState>,
    track_id: String,
    album: Option<String>,
    artist: Option<String>,
) -> Result<PlaybackState, PlaybackError> {
    if let (Some(album_name), Some(artist_name)) = (album, artist) {
        let album_tracks = state
            .library_service
            .tracks_by_album(&album_name, &artist_name)
            .await
            .map_err(PlaybackError::from)?;

        state
            .playback_service
            .play_album_tracks(album_tracks, &track_id)
            .map_err(PlaybackError::from)
    } else {
        let track = state
            .library_service
            .track_by_id(&track_id)
            .await
            .map_err(PlaybackError::from)?;
        state
            .playback_service
            .play_single_track(track)
            .map_err(PlaybackError::from)
    }
}
