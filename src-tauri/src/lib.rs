use std::sync::{Arc, Mutex};

use crate::player::{
    library::Library,
    playback::Playback,
    playback_driver::DefaultPlaybackDriver,
    track::Track,
};
use serde::{Deserialize, Serialize};
use tauri::{ipc::Channel, Builder, Emitter, Manager};

mod commands;
mod config;
mod player;

use commands::*;
use config::AppConfig;

pub struct AppState {
    pub playback: Arc<Mutex<Playback>>,
    pub progress_channel: Arc<Mutex<Option<Channel<ProgressEvent>>>>,
    pub config: Arc<Mutex<AppConfig>>,
    pub library: Arc<Mutex<Library>>,
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
pub struct ProgressEvent {
    pub position: f64,
    pub frames_played: u64,
    pub spectrum_data: Vec<f32>,
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
            get_library_tracks,
            select_track_from_queue
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
