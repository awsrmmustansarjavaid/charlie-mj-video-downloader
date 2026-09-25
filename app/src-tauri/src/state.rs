use crate::models::{BrowserMediaCapture, DownloadItem};
use std::{path::PathBuf, sync::Mutex};

pub struct AppState {
    pub downloads: Mutex<Vec<DownloadItem>>,
    pub captures: Mutex<Vec<BrowserMediaCapture>>,
    pub browser_watch: Mutex<bool>,
    pub data_dir: PathBuf,
}
