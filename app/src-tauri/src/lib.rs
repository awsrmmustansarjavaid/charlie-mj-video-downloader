mod commands;
mod database;
mod media;
mod models;
mod state;

use commands::*;
use state::AppState;
use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = AppState {
        downloads: Mutex::new(Vec::new()),
        data_dir: std::env::current_dir().unwrap_or_default(),
    };

    tauri::Builder::default()
        .manage(state)
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_notification::init())
        .invoke_handler(tauri::generate_handler![
            analyze_url,
            start_download,
            list_downloads,
            cancel_download,
            pause_download,
            resume_download
        ])
        .setup(|app| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_title("Charlie MJ Video Downloader");
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Charlie MJ Video Downloader");
}
