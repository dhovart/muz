use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::player::{
    library::Library,
    playback::{Playback, PlaybackState},
    playback_driver::DefaultPlaybackDriver,
};
use anyhow::Error;
use serde::{Deserialize, Serialize};
use tauri::{ipc::Channel, Builder, Manager, State};

mod player;

struct AppState {
    playback: Arc<Mutex<Playback>>,
    progress_channel: Arc<Mutex<Option<Channel<ProgressEvent>>>>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ProgressEvent {
    position_percent: f64,
}

#[derive(Deserialize, Debug)]
struct ControlPlaybackPayload {
    pub command: String,
    pub volume: Option<f32>,
    pub duration: Option<u64>, // in milliseconds
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
            .seek(Duration::from_millis(payload.duration.unwrap_or(0)))
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .setup(|app| {
            let library = Library::new(
                std::path::PathBuf::from("/System/Library/Sounds"),
                "Library".to_string(),
            );

            let progress_channel: Arc<Mutex<Option<Channel<ProgressEvent>>>> =
                Arc::new(Mutex::new(None));
            let progress_channel_clone = progress_channel.clone();

            let volume = 1.0; // fetch from some settings
            let playback_driver = DefaultPlaybackDriver::new(volume);
            let playback = Playback::create(Box::new(playback_driver), move |progress| {
                let event = ProgressEvent {
                    position_percent: progress,
                };
                if let Ok(channel_guard) = progress_channel_clone.lock() {
                    if let Some(ref channel) = *channel_guard {
                        let _ = channel.send(event);
                    }
                }
            });

            playback
                .lock()
                .unwrap()
                .enqueue_multiple(library.get_tracks());

            app.manage(AppState {
                playback,
                progress_channel,
            });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            control_playback,
            subscribe_to_progress
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
