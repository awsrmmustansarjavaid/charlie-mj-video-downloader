// ============================================================
// Charlie MJ Video Downloader - Application State
// ============================================================
// Purpose:
// - Defines the shared application state used by the Tauri app.
// - Stores download jobs, browser media captures, browser-watch
//   status, and the application data directory.
//
// This state is managed by Tauri and can be accessed by commands
// and other backend components during the application's lifetime.
// ============================================================

// Import the data models used by the application state.
use crate::models::{BrowserMediaCapture, DownloadItem};

// Import:
// - PathBuf: stores filesystem paths.
// - Mutex: provides safe shared access to mutable state.
use std::{path::PathBuf, sync::Mutex};


// ============================================================
// Application State
// ============================================================
// AppState is the central shared state container for the
// Charlie MJ Video Downloader backend.
//
// Tauri manages this structure using `.manage(state)` in lib.rs.
//
// Different parts of the application can access this state,
// including Tauri commands, IPC handlers, and download logic.
// ============================================================
pub struct AppState {

    // --------------------------------------------------------
    // Download Jobs
    // --------------------------------------------------------
    // Stores all download jobs currently known to the
    // application.
    //
    // Mutex protects the vector so multiple asynchronous
    // operations can safely access or modify it.
    pub downloads: Mutex<Vec<DownloadItem>>,

    // --------------------------------------------------------
    // Browser Media Captures
    // --------------------------------------------------------
    // Stores media streams detected by the Chrome extension.
    //
    // A capture can contain one or more video/audio streams
    // detected from a browser tab.
    pub captures: Mutex<Vec<BrowserMediaCapture>>,

    // --------------------------------------------------------
    // Browser Watch Status
    // --------------------------------------------------------
    // Controls whether browser media monitoring is enabled.
    //
    // true  = browser watching enabled
    // false = browser watching disabled
    //
    // Mutex allows this value to be safely changed while the
    // application is running.
    pub browser_watch: Mutex<bool>,

    // --------------------------------------------------------
    // Application Data Directory
    // --------------------------------------------------------
    // Stores the base filesystem directory used by the
    // application for local data and temporary files.
    //
    // PathBuf is used because it provides platform-independent
    // filesystem path handling.
    pub data_dir: PathBuf,
}