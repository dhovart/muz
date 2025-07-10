use std::{
    sync::OnceLock,
    sync::mpsc::{self, Sender},
    thread,
};
use tauri::Builder;

static AUDIO_SENDER: OnceLock<Sender<String>> = OnceLock::new();

#[tauri::command]
fn play() {
    println!("Playing sound...");
    let path = "/System/Library/Sounds/Submarine.aiff".to_string();

    if let Some(sender) = AUDIO_SENDER.get() {
        if let Err(err) = sender.send(path) {
            eprintln!("Audio thread has shut down: {}", err);
        }
    } else {
        eprintln!("Audio system not initialized!");
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let (tx, rx) = mpsc::channel::<String>();
    AUDIO_SENDER
        .set(tx)
        .expect("Audio sender initialized twice");

    thread::spawn(move || {
        let (mut manager, _backend) = awedio::start().expect("Failed to start audio manager");
        for path in rx {
            match awedio::sounds::open_file(&path) {
                Ok(file) => manager.play(file),
                Err(err) => eprintln!("Failed to open '{}': {}", path, err),
            }
        }
    });

    Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, play])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
