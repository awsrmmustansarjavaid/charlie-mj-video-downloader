use rusqlite::{Connection, Result};
use std::path::Path;

pub fn initialize(path: &Path) -> Result<()> {
    let connection = Connection::open(path)?;
    connection.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS downloads (
          id TEXT PRIMARY KEY,
          url TEXT NOT NULL,
          title TEXT NOT NULL,
          status TEXT NOT NULL,
          progress REAL NOT NULL DEFAULT 0,
          speed_bytes_per_second INTEGER NOT NULL DEFAULT 0,
          total_bytes INTEGER,
          downloaded_bytes INTEGER NOT NULL DEFAULT 0,
          output_path TEXT,
          error TEXT,
          created_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS browser_captures (
          capture_id TEXT PRIMARY KEY,
          source TEXT NOT NULL,
          page_url TEXT,
          title TEXT,
          payload_json TEXT NOT NULL,
          created_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS settings (
          key TEXT PRIMARY KEY,
          value TEXT NOT NULL
        );
        "#,
    )?;
    Ok(())
}
