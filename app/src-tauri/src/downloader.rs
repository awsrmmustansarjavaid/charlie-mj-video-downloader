// ============================================================
// Charlie MJ Video Downloader - Stream Downloader
// ============================================================
// Purpose:
// - Downloads individual media streams from HTTP/HTTPS URLs.
// - Supports browser-captured video and audio streams.
// - Writes downloaded data directly to a local file.
// - Creates temporary directories used during media processing.
//
// Main technologies:
// - reqwest     → HTTP client
// - Tokio       → asynchronous file/network operations
// - futures     → asynchronous stream processing
//
// Typical flow:
//
// Captured Video URL
//       ↓
// download_stream()
//       ↓
// HTTP Request
//       ↓
// Response Byte Stream
//       ↓
// Write to Temporary File
//       ↓
// FFmpeg Mux
// ============================================================

use futures_util::StreamExt;
use reqwest::Client;
use std::{
    // Path is used for filesystem locations.
    path::{Path, PathBuf},

    // Instant can be used for measuring download duration.
    time::Instant,
};
use tokio::{
    // Tokio asynchronous filesystem operations.
    fs,

    // AsyncWriteExt provides methods such as write_all()
// and flush() for asynchronous files.
    io::AsyncWriteExt,
};


// ============================================================
// Download Media Stream
// ============================================================
// Purpose:
// - Downloads a single video or audio stream.
// - Supports HTTP and HTTPS URLs.
// - Creates the destination directory if necessary.
// - Streams the response directly to disk.
//
// Parameters:
// - url  → Source media stream URL.
// - path → Destination file path.
//
// Returns:
// - Ok(u64) → Number of bytes downloaded.
// - Err(String) → Error description.
//
// This function is designed for browser-captured streams where
// video and audio may be downloaded separately.
// ============================================================

pub async fn download_stream(
    url: &str,
    path: &Path
) -> Result<u64, String> {

    // --------------------------------------------------------
    // Validate URL Scheme
    // --------------------------------------------------------
    // Parse the supplied URL and make sure it uses HTTP or
    // HTTPS.
    //
    // Other protocols are rejected for security and because
    // this downloader only supports web-based streams.
    if !matches!(
        url::Url::parse(url)
            .map_err(|e| e.to_string())?
            .scheme(),
        "http" | "https"
    ) {
        return Err(
            "Only HTTP/HTTPS streams are supported".into()
        );
    }


    // --------------------------------------------------------
    // Create Parent Directory
    // --------------------------------------------------------
    // If the destination file is inside a directory that does
    // not exist, create the required directory structure.
    //
    // Example:
    //
    // temp/job-id/video.tmp
    //
    // The "temp/job-id" directory will be created if necessary.
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .await
            .map_err(|e| e.to_string())?;
    }


    // --------------------------------------------------------
    // Create HTTP Client
    // --------------------------------------------------------
    // Build a reusable HTTP client for the stream request.
    //
    // A custom User-Agent identifies the application when
    // making the HTTP request.
    let client = Client::builder()
        .user_agent("Charlie-MJ-Video-Downloader/0.2")
        .build()
        .map_err(|e| e.to_string())?;


    // --------------------------------------------------------
    // Send HTTP GET Request
    // --------------------------------------------------------
    // Request the media stream from the supplied URL.
    //
    // send().await performs the asynchronous network request.
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| e.to_string())?;


    // --------------------------------------------------------
    // Validate HTTP Response
    // --------------------------------------------------------
    // Only continue when the server returns a successful
    // HTTP status code.
    //
    // Examples of successful responses:
    // - 200 OK
    // - Other 2xx responses
    //
    // Non-success responses are returned as errors.
    if !response.status().is_success() {
        return Err(
            format!(
                "Stream request failed: {}",
                response.status()
            )
        );
    }


    // --------------------------------------------------------
    // Create Destination File
    // --------------------------------------------------------
    // Create or overwrite the destination file.
    //
    // The downloaded stream will be written directly into this
    // file instead of loading the complete media into memory.
    let mut file = fs::File::create(path)
        .await
        .map_err(|e| e.to_string())?;


    // --------------------------------------------------------
    // Convert Response Into Async Byte Stream
    // --------------------------------------------------------
    // bytes_stream() allows the response body to be processed
    // chunk by chunk.
    //
    // This is more memory-efficient for large video files
    // because the entire file does not need to be stored in RAM.
    let mut stream = response.bytes_stream();


    // --------------------------------------------------------
    // Download Statistics
    // --------------------------------------------------------
    // Keep track of the total number of bytes received.
    let mut total = 0u64;


    // Start a timer for potential download-duration tracking.
    //
    // Currently the value is not used elsewhere in this
    // function, but it can later support speed or duration
    // calculations.
    let _started = Instant::now();


    // --------------------------------------------------------
    // Process Download Stream
    // --------------------------------------------------------
    // Read the HTTP response one chunk at a time until the
    // server has finished sending the stream.
    while let Some(chunk) = stream.next().await {

        // Convert any network/stream error into a String.
        let chunk = chunk
            .map_err(|e| e.to_string())?;


        // Write the received bytes directly to the output file.
        file.write_all(&chunk)
            .await
            .map_err(|e| e.to_string())?;


        // Add the size of this chunk to the total byte count.
        total += chunk.len() as u64;
    }


    // --------------------------------------------------------
    // Flush File
    // --------------------------------------------------------
    // Make sure buffered file data is written before returning.
    file.flush()
        .await
        .map_err(|e| e.to_string())?;


    // Return the total number of bytes successfully downloaded.
    Ok(total)
}


// ============================================================
// Create Temporary Stream Directory
// ============================================================
// Purpose:
// - Creates the path used to store temporary video/audio files.
//
// Directory structure:
//
// base/
//   └── temp/
//       └── job_id/
//           ├── video.tmp
//           └── audio.tmp
//
// Parameters:
// - base   → Application/base directory.
// - job_id → Unique download job ID.
//
// Returns:
// - PathBuf containing the temporary directory path.
//
// The directory itself is created later by download_stream()
// when the destination file is prepared.
// ============================================================

pub fn temp_pair_dir(
    base: &Path,
    job_id: &str
) -> PathBuf {

    // Build the temporary directory path:
    //
    // <base>/temp/<job_id>
    base.join("temp").join(job_id)
}