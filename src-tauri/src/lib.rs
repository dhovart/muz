use std::sync::{Arc, Mutex};

use crate::player::{playback::Playback, playback_driver::DefaultPlaybackDriver, track};
use tauri::{Builder, Manager, State};

mod player;

struct AppState {
    playback: Arc<Mutex<Playback>>,
}

#[tauri::command]
fn play(state: State<'_, AppState>) -> Result<String, ()> {
    let track = track::Track::new("/Users/denishovart/Dev/Diskus-Main/public/IncomingCall.wav");
    let mut playback = state.playback.lock().unwrap();
    let _ = playback.play(track);

    Ok("Track is playing".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .setup(|app| {
            // let library = library::Library::new(
            //     std::path::PathBuf::from("/System/Library/Sounds"),
            //     "My Music Library".to_string(),
            // );

            let playback_driver = DefaultPlaybackDriver::new();
            let playback = Playback::create_shared(Box::new(playback_driver));

            app.manage(AppState { playback });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![play])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
