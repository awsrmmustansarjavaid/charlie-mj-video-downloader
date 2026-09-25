// ============================================================
// Charlie MJ Video Downloader - Application Entry Point
// ============================================================
// Purpose:
// - Provides the executable entry point for the Tauri desktop app.
// - Starts the main application logic defined in lib.rs.
//
// Application flow:
//
// Windows
//   ↓
// main.rs
//   ↓
// lib.rs → run()
//   ↓
// Tauri Application
//   ↓
// React Frontend + Rust Backend
// ============================================================

// ------------------------------------------------------------
// Windows Subsystem Configuration
// ------------------------------------------------------------
// In release builds, prevent Windows from opening a separate
// console window for the desktop application.
//
// `cfg_attr` means:
// - Debug build     → console behavior remains available.
// - Release build   → run as a Windows GUI application.
//
// This is commonly used for Windows desktop applications built
// with Tauri.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


// ------------------------------------------------------------
// Rust Application Entry Point
// ------------------------------------------------------------
// Every Rust executable starts execution from the `main()`
// function.
//
// The actual application setup is kept inside the library
// module (`lib.rs`) so that the Tauri application logic can be
// organized separately from this minimal executable entry point.
fn main() {

    // --------------------------------------------------------
    // Start Charlie MJ Video Downloader
    // --------------------------------------------------------
    // Call the `run()` function defined in lib.rs.
    //
    // `run()` is responsible for:
    // - Creating AppState
    // - Configuring Tauri
    // - Registering Tauri plugins
    // - Registering frontend commands
    // - Starting the local IPC server
    // - Launching the Tauri application
    charlie_mj_video_downloader_lib::run();
}