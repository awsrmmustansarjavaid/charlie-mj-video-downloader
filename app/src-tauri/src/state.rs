use crate::models::DownloadItem;
use std::{path::PathBuf, sync::Mutex};

pub struct AppState {
    pub downloads: Mutex<Vec<DownloadItem>>,
    pub data_dir: PathBuf,
}
