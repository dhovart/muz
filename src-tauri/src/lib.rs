use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::player::{
    library::Library,
    playback::{self, Playback, PlaybackState},
    playback_driver::DefaultPlaybackDriver,
    track::Track,
};
use anyhow::Error;
use serde::{Deserialize, Serialize};
use tauri::{ipc::Channel, Builder, Emitter, Manager, State};

mod config;
mod player;

use config::AppConfig;

struct AppState {
    playback: Arc<Mutex<Playback>>,
    progress_channel: Arc<Mutex<Option<Channel<ProgressEvent>>>>,
    config: Arc<Mutex<AppConfig>>,
    library: Arc<Mutex<Library>>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct HistoryUpdateEvent {
    has_history: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct TrackChangedEvent {
    track: Option<Track>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct QueueChangedEvent {
    queue: Vec<Track>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ProgressEvent {
    position: f64,
    frames_played: u64,
    spectrum_data: Vec<f32>,
}

#[derive(Deserialize, Debug)]
struct ControlPlaybackPayload {
    pub command: String,
    pub volume: Option<f32>,
    pub seek_position: Option<u64>, // in milliseconds
}

#[tauri::command]
fn subscribe_to_progress(state: State<'_, AppState>, on_progress: Channel<ProgressEvent>) {
    let mut channel_guard = state.progress_channel.lock().unwrap();
    *channel_guard = Some(on_progress);
}

#[derive(Debug, Clone, Serialize)]
struct PlaybackError(String);

impl PlaybackError {
    fn from(err: Error) -> Self {
        PlaybackError(err.to_string())
    }
}

#[tauri::command]
fn control_playback(
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
fn get_library_path(state: State<'_, AppState>) -> Result<String, String> {
    let config = state.config.lock().map_err(|e| e.to_string())?;
    Ok(config.library_path.to_string_lossy().to_string())
}

#[tauri::command]
fn set_library_path(
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
fn rescan_library(state: State<'_, AppState>) -> Result<(), String> {
    let mut library = state.library.lock().map_err(|e| e.to_string())?;
    library.rescan();

    // Update playback queue with rescanned tracks
    let mut playback = state.playback.lock().map_err(|e| e.to_string())?;
    playback.clear_queue();
    playback.enqueue_multiple(library.get_tracks());

    Ok(())
}

#[tauri::command]
fn get_library_tracks(state: State<'_, AppState>) -> Result<Vec<Track>, String> {
    let library = state.library.lock().map_err(|e| e.to_string())?;
    Ok(library.get_tracks())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .setup(|app| {
            let config = AppConfig::load(app.handle()).unwrap_or_default();
            let library = Library::new(config.library_path.clone(), "Library".to_string());

            let progress_channel: Arc<Mutex<Option<Channel<ProgressEvent>>>> =
                Arc::new(Mutex::new(None));
            let progress_channel_clone = progress_channel.clone();

            let on_progress = move |progress, frames_played, spectrum_data| {
                let event = ProgressEvent {
                    position: progress,
                    frames_played,
                    spectrum_data,
                };
                if let Ok(channel_guard) = progress_channel_clone.lock() {
                    if let Some(ref channel) = *channel_guard {
                        let _ = channel.send(event);
                    }
                }
            };

            let app_handle = app.handle().clone();
            let app_handle_track = app_handle.clone();
            let app_handle_queue = app_handle.clone();

            let on_history_update = move |history: &Vec<Track>, current_track: Option<&Track>| {
                let event = HistoryUpdateEvent {
                    has_history: history.len() > 1
                        || (history.len() == 1 && history.last() != current_track),
                };
                let _ = app_handle.emit("history-update", event);
            };

            let on_track_changed = move |track: Option<&Track>| {
                let event = TrackChangedEvent {
                    track: track.cloned(),
                };
                let _ = app_handle_track.emit("track-changed", event);
            };

            let on_queue_changed = move |queue: &Vec<Track>| {
                let event = QueueChangedEvent {
                    queue: queue.clone(),
                };
                let _ = app_handle_queue.emit("queue-changed", event);
            };

            let volume = 1.0; // fetch from some settings
            let playback_driver = DefaultPlaybackDriver::new(volume);
            let playback = Playback::create(
                Box::new(playback_driver),
                on_progress,
                on_history_update,
                on_track_changed,
                on_queue_changed,
            );

            let tracks = library.get_tracks();
            playback.lock().unwrap().enqueue_multiple(tracks.clone());

            // Emit initial events
            let initial_track_event = TrackChangedEvent { track: None };
            let _ = app.emit("track-changed", initial_track_event);

            let initial_queue_event = QueueChangedEvent { queue: tracks };
            let _ = app.emit("queue-changed", initial_queue_event);

            app.manage(AppState {
                playback,
                progress_channel,
                config: Arc::new(Mutex::new(config)),
                library: Arc::new(Mutex::new(library)),
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            control_playback,
            subscribe_to_progress,
            get_library_path,
            set_library_path,
            rescan_library,
            get_library_tracks
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
