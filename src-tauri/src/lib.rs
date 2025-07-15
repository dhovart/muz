use std::sync::{Arc, Mutex};

use crate::player::{library::Library, playback::Playback, playback_driver::DefaultPlaybackDriver};
use tauri::{Builder, Manager, State};

mod player;

struct AppState {
    playback: Arc<Mutex<Playback>>,
}

// FIXME create a single command for sending control commands to the player

#[tauri::command]
fn play(state: State<'_, AppState>) -> Result<String, String> {
    state
        .playback
        .lock()
        .unwrap()
        .play()
        .map_err(|e| e.to_string())?;
    Ok("Play".to_string())
}

#[tauri::command]
fn pause(state: State<'_, AppState>) -> Result<String, String> {
    state
        .playback
        .lock()
        .unwrap()
        .pause()
        .map_err(|e| e.to_string())?;
    Ok("Paused".to_string())
}

#[tauri::command]
fn next_track(state: State<'_, AppState>) -> Result<String, String> {
    state
        .playback
        .lock()
        .unwrap()
        .next()
        .map_err(|e| e.to_string())?;
    Ok("Next track".to_string())
}

#[tauri::command]
fn previous_track(state: State<'_, AppState>) -> Result<String, String> {
    state
        .playback
        .lock()
        .unwrap()
        .previous()
        .map_err(|e| e.to_string())?;
    Ok("Previous track".to_string())
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
            let playback = Playback::create(Box::new(playback_driver));

            playback
                .lock()
                .unwrap()
                .enqueue_multiple(library.get_tracks());

            app.manage(AppState { playback });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            play,
            pause,
            next_track,
            previous_track
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
