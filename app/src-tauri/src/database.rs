// ============================================================
// Charlie MJ Video Downloader - Database Initialization
// ============================================================
// Purpose:
// - Initializes the application's SQLite database.
// - Creates the required database tables if they do not exist.
// - Stores download history, browser captures, and settings.
//
// Database technology:
// - SQLite
// - rusqlite
//
// Main tables:
//
// downloads
//     ↓
// Stores download jobs and their progress/status.
//
// browser_captures
//     ↓
// Stores media detected by the browser extension.
//
// settings
//     ↓
// Stores application configuration values.
//
// The CREATE TABLE IF NOT EXISTS statements make this function
// safe to run multiple times without recreating existing tables.
// ============================================================

use rusqlite::{Connection, Result};
use std::path::Path;


// ============================================================
// Initialize Database
// ============================================================
// Purpose:
// - Opens or creates the SQLite database file.
// - Creates all required application tables.
//
// Parameters:
// - path: Location of the SQLite database file.
//
// Returns:
// - Ok(()) when initialization succeeds.
// - rusqlite::Result error if the database cannot be opened
//   or the SQL statements fail.
// ============================================================

pub fn initialize(path: &Path) -> Result<()> {

    // --------------------------------------------------------
    // Open Database Connection
    // --------------------------------------------------------
    // Connection::open() opens the existing SQLite database
    // or creates a new database file if it does not exist.
    //
    // The ? operator immediately returns an error if the
    // database cannot be opened.
    let connection = Connection::open(path)?;


    // --------------------------------------------------------
    // Create Database Tables
    // --------------------------------------------------------
    // execute_batch() executes multiple SQL statements as one
    // batch.
    //
    // CREATE TABLE IF NOT EXISTS ensures that existing tables
    // are preserved when the application starts again.
    connection.execute_batch(
        r#"

        // ====================================================
        // Downloads Table
        // ====================================================
        // Stores information about downloads managed by the
        // Charlie MJ Video Downloader application.
        //
        // Each download has a unique ID and contains information
        // about its source, status, progress, speed, output file,
        // and possible errors.
        CREATE TABLE IF NOT EXISTS downloads (

          // Unique identifier for the download job.
          id TEXT PRIMARY KEY,

          // Original media URL.
          url TEXT NOT NULL,

          // Media title or display name.
          title TEXT NOT NULL,

          // Current download status.
          // Example: queued, downloading, paused, cancelled.
          status TEXT NOT NULL,

          // Download progress percentage/value.
          // Defaults to 0 when a new job is created.
          progress REAL NOT NULL DEFAULT 0,

          // Current download speed in bytes per second.
          speed_bytes_per_second INTEGER NOT NULL DEFAULT 0,

          // Total media size in bytes.
          // NULL means the total size is not known.
          total_bytes INTEGER,

          // Number of bytes downloaded so far.
          downloaded_bytes INTEGER NOT NULL DEFAULT 0,

          // Final output file location.
          // NULL when the output path has not been assigned.
          output_path TEXT,

          // Error message if the download fails.
          // NULL when there is no error.
          error TEXT,

          // Time when the download job was created.
          created_at TEXT NOT NULL
        );


        // ====================================================
        // Browser Captures Table
        // ====================================================
        // Stores media information captured from the browser
        // extension.
        //
        // A capture may contain one or more video/audio streams.
        CREATE TABLE IF NOT EXISTS browser_captures (

          // Unique identifier for the browser capture.
          capture_id TEXT PRIMARY KEY,

          // Source of the captured media.
          // Example: browser or google_drive.
          source TEXT NOT NULL,

          // URL of the browser page where the media was found.
          page_url TEXT,

          // Page or media title.
          title TEXT,

          // Complete captured-media information stored as JSON.
          //
          // JSON allows multiple stream details and other
          // metadata to be preserved without requiring a
          // separate table for every stream.
          payload_json TEXT NOT NULL,

          // Time when the browser capture was created.
          created_at TEXT NOT NULL
        );


        // ====================================================
        // Settings Table
        // ====================================================
        // Stores application configuration as key/value pairs.
        //
        // Example:
        //
        // key             value
        // --------------------------------
        // download_path   C:\Downloads
        // browser_watch   true
        CREATE TABLE IF NOT EXISTS settings (

          // Unique setting name.
          key TEXT PRIMARY KEY,

          // Setting value stored as text.
          value TEXT NOT NULL
        );

        "#,
    )?;


    // --------------------------------------------------------
    // Initialization Successful
    // --------------------------------------------------------
    // All required database tables have now been created or
    // confirmed to already exist.
    Ok(())
}