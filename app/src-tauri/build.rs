// ============================================================
// Charlie MJ Video Downloader - Tauri Build Script
// ============================================================
// Purpose:
// - Provides the build script required by Tauri.
// - Runs Tauri's build-time setup process.
// - Helps Tauri prepare the Rust application during compilation.
//
// This file is executed by Cargo during the build process.
//
// Typical location:
// app/src-tauri/build.rs
//
// Build flow:
//
// Cargo
//   ↓
// build.rs
//   ↓
// tauri_build::build()
//   ↓
// Tauri build configuration
//   ↓
// Charlie MJ Desktop Application
// ============================================================


fn main() {

    // --------------------------------------------------------
    // Run Tauri Build Process
    // --------------------------------------------------------
    // tauri_build::build() performs the required build-time
    // setup for the Tauri application.
    //
    // It reads the Tauri configuration and generates the
    // necessary build information used when compiling the
    // desktop application.
    //
    // This is normally required for a Tauri application and
    // should remain in the build.rs file.
    tauri_build::build()
}