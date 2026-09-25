// ============================================================
// Charlie MJ Video Downloader - Data Models
// ============================================================
// Purpose:
// - Defines the main data structures used by the application.
// - Provides a common data format between Rust and the frontend.
// - Represents media information, downloadable formats,
//   browser-captured streams, and download jobs.
//
// Serialization flow:
//
// yt-dlp JSON
//     ↓
// Rust Models
//     ↓
// Serde Serialization
//     ↓
// Tauri / JSON
//     ↓
// React Frontend
//
// Browser capture flow:
//
// Chrome Extension
//     ↓
// Native Host
//     ↓
// JSON
//     ↓
// Rust Models
//     ↓
// React Frontend
// ============================================================

// Serde provides serialization and deserialization support.
//
// Serialize:
// Rust struct → JSON
//
// Deserialize:
// JSON → Rust struct
use serde::{Deserialize, Serialize};


// ============================================================
// Media Format
// ============================================================
// Represents one downloadable media format returned by yt-dlp.
//
// A media URL can provide multiple formats, for example:
//
// - 1080p video
// - 720p video
// - 480p video
// - Audio-only
// - Different codecs
// - Different frame rates
//
// Each format is stored as a MediaFormat object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaFormat {

    // yt-dlp format identifier.
    //
    // Example:
    // "137", "140", etc.
    pub id: String,

    // Container/file extension.
    //
    // Examples:
    // mp4, webm, m4a
    pub extension: String,

    // Video resolution when available.
    //
    // Example:
    // "1920x1080"
    pub resolution: Option<String>,

    // Video frame rate when available.
    //
    // Example:
    // 30.0 or 60.0 FPS
    pub fps: Option<f64>,

    // Exact file size when available.
    //
    // Stored in bytes.
    pub filesize: Option<u64>,

    // Video codec.
    //
    // Examples:
    // avc1, vp9, av01
    pub vcodec: Option<String>,

    // Audio codec.
    //
    // Examples:
    // aac, opus, mp4a
    pub acodec: Option<String>,

    // Audio bitrate in kbps when available.
    pub abr: Option<f64>,

    // Additional format information supplied by yt-dlp.
    //
    // This may contain information such as quality or codec
    // notes.
    pub note: Option<String>,
}


// ============================================================
// Media Information
// ============================================================
// Represents the complete metadata returned when analyzing
// a media URL with yt-dlp.
//
// This structure is normally created by media::analyze()
// and returned to the frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaInfo {

    // Unique media identifier provided by yt-dlp.
    pub id: String,

    // Media title.
    pub title: String,

    // Uploader, channel, or creator name when available.
    pub uploader: Option<String>,

    // Media duration in seconds when available.
    pub duration: Option<f64>,

    // Thumbnail image URL when available.
    pub thumbnail: Option<String>,

    // Original webpage URL.
    //
    // Rust uses snake_case:
    //     webpage_url
    //
    // The frontend receives camelCase:
    //     webpageUrl
    //
    // This conversion is handled by Serde.
    #[serde(rename = "webpageUrl")]
    pub webpage_url: String,

    // All formats discovered by yt-dlp.
    pub formats: Vec<MediaFormat>,

    // Available subtitle language codes.
    //
    // Examples:
    // "en", "tr", "ur"
    pub subtitles: Vec<String>,
}


// ============================================================
// Captured Stream
// ============================================================
// Represents one video or audio stream captured from the
// browser by the Chrome extension.
//
// A browser page may expose separate:
//
// - Video stream
// - Audio stream
//
// These streams can later be downloaded and combined using
// downloader.rs and ffmpeg.rs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapturedStream {

    // Unique identifier for this captured stream.
    pub id: String,

    // Source of the captured stream.
    //
    // Examples:
    // "browser"
    // "google_drive"
    pub source: String,

    // Browser tab ID where the stream was detected.
    //
    // Rust field:
    //     tab_id
    //
    // JSON field:
    //     tabId
    #[serde(rename = "tabId")]
    pub tab_id: Option<i64>,

    // URL of the webpage where the stream was detected.
    //
    // Rust field:
    //     page_url
    //
    // JSON field:
    //     pageUrl
    #[serde(rename = "pageUrl")]
    pub page_url: Option<String>,

    // Page or media title when available.
    pub title: Option<String>,

    // Type of captured stream.
    //
    // Expected values:
    // "video"
    // "audio"
    //
    // The Rust field is called `stream_type`, while the JSON
    // representation uses `type`.
    #[serde(rename = "type")]
    pub stream_type: String,

    // Direct URL of the captured media stream.
    pub url: String,

    // MIME type of the stream.
    //
    // Examples:
    // video/mp4
    // audio/mp4
    // video/webm
    pub mime: Option<String>,

    // Quality information when available.
    pub quality: Option<String>,

    // Video width in pixels.
    pub width: Option<u32>,

    // Video height in pixels.
    pub height: Option<u32>,

    // Video frame rate.
    pub fps: Option<f64>,

    // Stream bitrate in bits per second when available.
    pub bitrate: Option<u64>,

    // Stream size in bytes when available.
    pub size: Option<u64>,

    // Stream duration in seconds when available.
    pub duration: Option<f64>,
}


// ============================================================
// Browser Media Capture
// ============================================================
// Represents a complete browser media detection event.
//
// One BrowserMediaCapture can contain multiple captured
// streams, such as:
//
// - Video stream
// - Audio stream
//
// This is the main data structure passed from the browser
// capture system to the Tauri application.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserMediaCapture {

    // Unique identifier for the complete capture event.
    //
    // Rust:
    //     capture_id
    //
    // JSON:
    //     captureId
    #[serde(rename = "captureId")]
    pub capture_id: String,

    // Source of the capture.
    //
    // Examples:
    // "browser"
    // "google_drive"
    pub source: String,

    // Webpage URL associated with the capture.
    //
    // JSON uses:
    // pageUrl
    #[serde(rename = "pageUrl")]
    pub page_url: Option<String>,

    // Page or media title when available.
    pub title: Option<String>,

    // Video/audio streams detected during this capture.
    pub streams: Vec<CapturedStream>,

    // Time when the capture was created.
    //
    // Stored as a string, normally using an RFC 3339 / ISO-8601
    // timestamp generated by the backend.
    //
    // JSON uses:
    // createdAt
    #[serde(rename = "createdAt")]
    pub created_at: String,
}


// ============================================================
// Download Item
// ============================================================
// Represents a download job managed by the application.
//
// This structure is used to track information such as:
//
// - Download ID
// - URL
// - Title
// - Status
// - Progress
// - Download speed
// - File size
// - Output location
// - Errors
//
// It can be exposed to the React frontend to display the
// download manager UI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadItem {

    // Unique identifier for the download job.
    pub id: String,

    // Source media URL.
    pub url: String,

    // Download title.
    pub title: String,

    // Current download status.
    //
    // Examples:
    // queued
    // downloading
    // paused
    // cancelled
    // completed
    // failed
    pub status: String,

    // Download progress.
    //
    // Usually represented as a percentage or progress value
    // between 0 and 100, depending on how the backend updates it.
    pub progress: f64,

    // Current download speed in bytes per second.
    //
    // JSON uses:
    // speedBytesPerSecond
    #[serde(rename = "speedBytesPerSecond")]
    pub speed_bytes_per_second: u64,

    // Total expected download size in bytes when known.
    //
    // JSON uses:
    // totalBytes
    #[serde(rename = "totalBytes")]
    pub total_bytes: Option<u64>,

    // Number of bytes downloaded so far.
    //
    // JSON uses:
    // downloadedBytes
    #[serde(rename = "downloadedBytes")]
    pub downloaded_bytes: u64,

    // Final or temporary output file path when available.
    //
    // JSON uses:
    // outputPath
    #[serde(rename = "outputPath")]
    pub output_path: Option<String>,

    // Error message when the download fails.
    pub error: Option<String>,

    // Time when the download job was created.
    //
    // JSON uses:
    // createdAt
    #[serde(rename = "createdAt")]
    pub created_at: String,
}