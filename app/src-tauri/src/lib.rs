// ============================================================
// Charlie MJ Video Downloader - Tauri Application Library
// ============================================================
// Purpose:
// - Defines the Rust modules used by the desktop application.
// - Creates and manages the shared application state.
// - Registers Tauri plugins.
// - Registers frontend-to-Rust commands.
// - Starts the local IPC server.
// - Builds and launches the Tauri desktop application.
//
// Main architecture:
//
// React / TypeScript Frontend
//          ↓
//     Tauri Commands
//          ↓
//      commands.rs
//          ↓
// Rust Backend Modules
//   ├── media.rs
//   ├── downloader.rs
//   ├── ffmpeg.rs
//   ├── database.rs
//   └── browser_capture.rs
//
// Browser capture flow:
//
// Chrome Extension
//       ↓
// Native Messaging Host
//       ↓
//      ipc.rs
//       ↓
// browser_capture.rs
//       ↓
//    AppState
//       ↓
// React Frontend
// ============================================================

// ------------------------------------------------------------
// Application Modules
// ------------------------------------------------------------
// Each `mod` statement registers a Rust module used by the
// Charlie MJ Video Downloader backend.

mod browser_capture; // Processes media captured by the browser.
mod commands;        // Tauri commands callable from the frontend.
mod database;        // SQLite database initialization and storage.
mod downloader;      // Direct HTTP media stream downloading.
mod ffmpeg;          // FFmpeg muxing, conversion, and verification.
mod ipc;             // Local HTTP IPC server for browser communication.
mod media;           // Media URL analysis and downloader integration.
mod models;          // Shared Rust data structures/models.
mod state;           // Shared application state.

// Shared application state used across Tauri commands and
// backend services.
use state::AppState;

// Standard library types used to create the initial application
// state and provide thread-safe shared storage.
use std::{path::PathBuf, sync::Mutex};

// Tauri Manager trait provides access to managed application
// state and application windows.
use tauri::Manager;

// ------------------------------------------------------------
// Tauri Application Entry Point
// ------------------------------------------------------------
// `run()` creates and starts the Charlie MJ Video Downloader
// desktop application.
//
// `cfg_attr` allows Tauri to use the appropriate mobile entry
// point when the application is compiled for a mobile target.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    // --------------------------------------------------------
    // Initialize Shared Application State
    // --------------------------------------------------------
    // AppState stores data that needs to be shared between
    // different parts of the Tauri backend.
    //
    // Mutex is used so multiple asynchronous tasks or commands
    // can safely access and modify the collections.
    let state = AppState {

        // Stores download jobs currently known to the
        // application.
        downloads: Mutex::new(Vec::new()),

        // Stores media captures received from the browser.
        captures: Mutex::new(Vec::new()),

        // Controls whether browser media monitoring is enabled.
        browser_watch: Mutex::new(false),

        // Base directory used by the application for local
        // files and temporary download data.
        //
        // If the current directory cannot be determined,
        // the application falls back to the current path ".".
        data_dir: std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from(".")),
    };

    // --------------------------------------------------------
    // Build Tauri Application
    // --------------------------------------------------------
    // Tauri::Builder configures the desktop application,
    // registers shared state, plugins, commands, and startup
    // logic before launching the application.
    tauri::Builder::default()

        // Make AppState available to Tauri commands and other
        // backend components through Tauri's managed state.
        .manage(state)

        // ----------------------------------------------------
        // Tauri Dialog Plugin
        // ----------------------------------------------------
        // Provides native dialogs such as file/folder selection
        // and confirmation dialogs.
        .plugin(tauri_plugin_dialog::init())

        // ----------------------------------------------------
        // Tauri File System Plugin
        // ----------------------------------------------------
        // Provides controlled filesystem functionality for the
        // Tauri application.
        .plugin(tauri_plugin_fs::init())

        // ----------------------------------------------------
        // Tauri Notification Plugin
        // ----------------------------------------------------
        // Enables desktop notifications for events such as
        // completed downloads or application messages.
        .plugin(tauri_plugin_notification::init())

        // ----------------------------------------------------
        // Register Frontend Commands
        // ----------------------------------------------------
        // These Rust functions become callable from the
        // React/TypeScript frontend using Tauri's invoke API.
        //
        // Example frontend flow:
        //
        // React
        //   ↓
        // invoke("analyze_url")
        //   ↓
        // commands::analyze_url()
        //   ↓
        // Rust backend
        .invoke_handler(tauri::generate_handler![

            // Analyze a media URL and return media information.
            commands::analyze_url,

            // Start a normal URL-based download.
            commands::start_download,

            // Start a download using browser-captured video/audio
            // streams.
            commands::start_captured_download,

            // Return the current list of download jobs.
            commands::list_downloads,

            // Return the browser media captures currently stored
            // in application state.
            commands::list_browser_captures,

            // Mark a download as cancelled.
            commands::cancel_download,

            // Mark a download as paused.
            commands::pause_download,

            // Mark a download as resumed/downloading.
            commands::resume_download,

            // Enable or disable browser media monitoring.
            commands::set_browser_watch
        ])

        // ----------------------------------------------------
        // Application Startup / Setup
        // ----------------------------------------------------
        // The setup callback runs after Tauri has initialized
        // the application but before the application starts
        // serving normal user interactions.
        .setup(|app| {

            // ------------------------------------------------
            // Configure Main Window
            // ------------------------------------------------
            // Find the main Tauri window using the label
            // "main".
            //
            // If the window exists, set its native window title.
            if let Some(window) = app.get_webview_window("main") {

                // Set the application window title.
                let _ = window.set_title(
                    "Charlie MJ Video Downloader"
                );
            }

            // ------------------------------------------------
            // Start Local IPC Server
            // ------------------------------------------------
            // Clone the Tauri application handle so it can be
            // moved safely into the asynchronous IPC task.
            let handle = app.handle().clone();

            // Start the local IPC server in a background task.
            //
            // The IPC server listens on:
            //
            // 127.0.0.1:47821
            //
            // and receives media detection messages from the
            // Chrome Native Messaging Host.
            tauri::async_runtime::spawn(async move {

                // Start the IPC server.
                //
                // The server normally keeps running for the
                // lifetime of the application.
                //
                // Any returned error is intentionally ignored
                // by the current implementation.
                let _ = ipc::start(handle).await;
            });

            // Indicate that application setup completed
            // successfully.
            Ok(())
        })

        // ----------------------------------------------------
        // Start Tauri Application
        // ----------------------------------------------------
        // Generate the Tauri runtime context from tauri.conf.json
        // and launch the desktop application.
        //
        // If the application cannot start, terminate with the
        // specified error message.
        .run(tauri::generate_context!())
        .expect(
            "error while running Charlie MJ Video Downloader"
        );
}