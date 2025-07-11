use std::sync::Arc;

use crate::player::{playback::Playback, track};
use tauri::{Builder, Manager, State};
use tokio::sync::Mutex;

mod player;

struct AppState {
    playback: Arc<Mutex<Playback>>,
}

#[tauri::command]
async fn play(state: State<'_, AppState>) -> Result<String, ()> {
    println!("Playing track... ");
    let track = track::Track::new("/System/Library/Sounds/Submarine.aiff");
    let mut playback = state.playback.lock().await;
    let _ = playback.play(track);

    Ok("Track is playing".to_string())
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .setup(|app| {
            // let library = library::Library::new(
            //     std::path::PathBuf::from("/System/Library/Sounds"),
            //     "My Music Library".to_string(),
            // );

            let app_handle = app.handle().clone();

            let playback_driver = player::playback_driver::DefaultPlaybackDriver::new();
            let playback = player::playback::Playback::create_shared(Box::new(playback_driver));

            app.manage(AppState { playback });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, play])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
