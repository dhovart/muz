use std::sync::{Arc, Mutex};

use crate::player::{library::Library, playback::Playback, playback_driver::DefaultPlaybackDriver};
use serde::{Deserialize, Serialize};
use tauri::{Builder, Manager, State};

mod player;

struct AppState {
    playback: Arc<Mutex<Playback>>,
}

#[derive(Clone, Serialize)]
struct ProgressEvent {
    position_percent: f64,
}

#[derive(Deserialize, Debug)]
struct ControlPlaybackPayload {
    pub command: String,
    pub volume: Option<f32>,
}

#[tauri::command]
fn control_playback(
    state: State<'_, AppState>,
    payload: ControlPlaybackPayload,
) -> Result<(), String> {
    // FIXME return type. Have Playback methods return meaningful result
    let mut playback = state.playback.lock().unwrap();

    match payload.command.as_str() {
        "Play" => playback.play().map_err(|e| e.to_string()),
        "Pause" => playback.pause().map_err(|e| e.to_string()),
        "Next" => playback.next().map_err(|e| e.to_string()),
        "Previous" => playback.previous().map_err(|e| e.to_string()),
        "SetVolume" => {
            if let Some(volume) = payload.volume {
                playback.set_volume(volume).map_err(|e| e.to_string())
            } else {
                Err("Invalid payload".to_string())
            }
        }
        _ => Err("Unknown action".to_string()),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .setup(|app| {
            let library = Library::new(
                std::path::PathBuf::from("/System/Library/Sounds"),
                "My Music Library".to_string(),
            );

            let playback_driver = DefaultPlaybackDriver::new();
            let playback = Playback::create(Box::new(playback_driver), |progress| {
                let event = ProgressEvent {
                    position_percent: progress as f64,
                };
                println!("Playback progress: {}%", event.position_percent);
            });

            playback
                .lock()
                .unwrap()
                .enqueue_multiple(library.get_tracks());

            app.manage(AppState { playback });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![control_playback])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
