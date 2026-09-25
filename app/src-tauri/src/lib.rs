mod browser_capture;
mod commands;
mod database;
mod downloader;
mod ffmpeg;
mod ipc;
mod media;
mod models;
mod state;

use state::AppState;
use std::{path::PathBuf, sync::Mutex};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = AppState {
        downloads: Mutex::new(Vec::new()),
        captures: Mutex::new(Vec::new()),
        browser_watch: Mutex::new(false),
        data_dir: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
    };

    tauri::Builder::default()
        .manage(state)
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_notification::init())
        .invoke_handler(tauri::generate_handler![
            commands::analyze_url,
            commands::start_download,
            commands::start_captured_download,
            commands::list_downloads,
            commands::list_browser_captures,
            commands::cancel_download,
            commands::pause_download,
            commands::resume_download,
            commands::set_browser_watch
        ])
        .setup(|app| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_title("Charlie MJ Video Downloader");
            }

            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let _ = ipc::start(handle).await;
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Charlie MJ Video Downloader");
}
