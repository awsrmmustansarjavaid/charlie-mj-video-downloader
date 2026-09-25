use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaFormat {
    pub id: String,
    pub extension: String,
    pub resolution: Option<String>,
    pub fps: Option<f64>,
    pub filesize: Option<u64>,
    pub vcodec: Option<String>,
    pub acodec: Option<String>,
    pub abr: Option<f64>,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaInfo {
    pub id: String,
    pub title: String,
    pub uploader: Option<String>,
    pub duration: Option<f64>,
    pub thumbnail: Option<String>,
    #[serde(rename = "webpageUrl")]
    pub webpage_url: String,
    pub formats: Vec<MediaFormat>,
    pub subtitles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapturedStream {
    pub id: String,
    pub source: String,
    #[serde(rename = "tabId")]
    pub tab_id: Option<i64>,
    #[serde(rename = "pageUrl")]
    pub page_url: Option<String>,
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub stream_type: String,
    pub url: String,
    pub mime: Option<String>,
    pub quality: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub fps: Option<f64>,
    pub bitrate: Option<u64>,
    pub size: Option<u64>,
    pub duration: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserMediaCapture {
    #[serde(rename = "captureId")]
    pub capture_id: String,
    pub source: String,
    #[serde(rename = "pageUrl")]
    pub page_url: Option<String>,
    pub title: Option<String>,
    pub streams: Vec<CapturedStream>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadItem {
    pub id: String,
    pub url: String,
    pub title: String,
    pub status: String,
    pub progress: f64,
    #[serde(rename = "speedBytesPerSecond")]
    pub speed_bytes_per_second: u64,
    #[serde(rename = "totalBytes")]
    pub total_bytes: Option<u64>,
    #[serde(rename = "downloadedBytes")]
    pub downloaded_bytes: u64,
    #[serde(rename = "outputPath")]
    pub output_path: Option<String>,
    pub error: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}
