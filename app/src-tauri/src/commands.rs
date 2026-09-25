// ============================================================
// Charlie MJ Video Downloader - Tauri Commands
// ============================================================
// Purpose:
// - Provides commands callable from the frontend.
// - Starts and manages downloads.
// - Handles browser-captured media streams.
// - Provides download and browser-capture state to the UI.
// - Controls pause, resume, cancel, and browser watching.
//
// Frontend flow:
//
// React / TypeScript
//       ↓
// Tauri Command
//       ↓
// commands.rs
//       ↓
// Media / Downloader / FFmpeg
//       ↓
// Downloaded Video
// ============================================================

use crate::{
    // Download handling utilities.
    downloader,

    // FFmpeg operations such as muxing and verification.
    ffmpeg,

    // Media analysis and download functionality.
    media,

    // Application data models.
    models::{BrowserMediaCapture, DownloadItem},

    // Shared application state.
    state::AppState,
};

use chrono::Utc;
use std::path::PathBuf;
use tauri::{AppHandle, Manager, State};
use uuid::Uuid;


// ============================================================
// Analyze URL
// ============================================================
// Purpose:
// - Analyzes a media URL.
// - Extracts information such as available formats,
//   media type, title, duration, and other metadata.
//
// Called from the frontend through Tauri.
// ============================================================

#[tauri::command]
pub async fn analyze_url(url: String) -> Result<crate::models::MediaInfo, String> {

    // Make sure the URL uses HTTP or HTTPS.
    validate_url(&url)?;

    // Pass the URL to the media analysis module.
    media::analyze(&url).await
}


// ============================================================
// Start Normal URL Download
// ============================================================
// Purpose:
// - Creates a new download job.
// - Starts downloading the supplied URL in the background.
// - Optionally accepts a format ID and output template.
//
// The actual download runs inside a Tokio background task so
// the Tauri frontend is not blocked while the download runs.
// ============================================================

#[tauri::command]
pub async fn start_download(
    url: String,
    format_id: Option<String>,
    output_template: Option<String>,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<String, String> {

    // Validate the supplied URL before starting the job.
    validate_url(&url)?;

    // Create a download entry in the application state.
    let id = create_job(&url, &url, &state)?;

    // Always save normal URL downloads in the user's Downloads
    // folder unless the frontend explicitly supplied a template.
    let template = output_template.or_else(|| {
        dirs::download_dir().map(|dir| {
            dir.join("%(title)s.%(ext)s")
                .to_string_lossy()
                .to_string()
        })
    });

    // Start the actual media download in a background task.
    // The previous implementation discarded the result, which left
    // every job permanently at `queued / 0% / unknown`.
    let job_id = id.clone();
    let download_url = url.clone();
    tokio::spawn(async move {
        let state = app.state::<AppState>();
        set_job_status(state.inner(), &job_id, "downloading", 0.0, None, None);

        let result = media::download(
            &download_url,
            format_id.as_deref(),
            template.as_deref()
        ).await;

        match result {
            Ok(output_path) => {
                set_job_status(
                    state.inner(),
                    &job_id,
                    "completed",
                    100.0,
                    Some(output_path),
                    None
                );
            }
            Err(error) => {
                set_job_status(
                    state.inner(),
                    &job_id,
                    "failed",
                    0.0,
                    None,
                    Some(error)
                );
            }
        }
    });

    Ok(id)
}


// ============================================================
// Start Browser-Captured Download
// ============================================================
// Purpose:
// - Downloads media streams detected by the Chrome extension.
// - Supports separate video and optional audio streams.
// - Downloads the streams separately.
// - Uses FFmpeg to combine/mux them into the final MP4 file.
// - Verifies the final output file.
//
// Flow:
//
// Chrome Extension
//       ↓
// Native Messaging Host
//       ↓
// BrowserMediaCapture
//       ↓
// start_captured_download()
//       ↓
// Download Video + Audio
//       ↓
// FFmpeg Mux
//       ↓
// Verify MP4
// ============================================================

#[tauri::command]
pub async fn start_captured_download(
    capture_id: String,
    video_stream_id: String,
    audio_stream_id: Option<String>,
    output_path: Option<String>,
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<String, String> {

    // --------------------------------------------------------
    // Find the Requested Browser Capture
    // --------------------------------------------------------
    // Lock the browser capture state and search for the
    // capture using the ID supplied by the frontend.
    let capture = state.captures.lock()
        .map_err(|_| "capture state unavailable")?
        .iter()
        .find(|x| x.capture_id == capture_id)
        .cloned()
        .ok_or("Capture no longer exists")?;


    // --------------------------------------------------------
    // Find the Selected Video Stream
    // --------------------------------------------------------
    // Search inside the selected browser capture for the
    // requested video stream.
    let video = capture.streams.iter()
        .find(|x| x.id == video_stream_id)
        .cloned()
        .ok_or("Video stream not found")?;


    // --------------------------------------------------------
    // Find the Optional Audio Stream
    // --------------------------------------------------------
    // Audio may be optional because some media streams already
    // contain both video and audio.
    //
    // If an audio stream ID was supplied, find that stream.
    // Otherwise, continue without a separate audio stream.
    let audio = audio_stream_id.and_then(|id|
        capture.streams.iter()
            .find(|x| x.id == id)
            .cloned()
    );


    // --------------------------------------------------------
    // Create Download Job Information
    // --------------------------------------------------------

    // Use the captured page title as the default filename.
    // If no title is available, use a generic application name.
    let title = capture
        .title
        .clone()
        .unwrap_or_else(|| "Charlie MJ Video".into());

    // Create the tracked download job before starting the background task.
    let job_id = create_job(&video.url, &title, &state)?;

    // Use the user's Downloads directory for completed browser-capture
    // downloads instead of the process working directory. Installed
    // Windows applications may start with an unrelated working directory.
    let base = dirs::download_dir()
        .or_else(dirs::data_local_dir)
        .unwrap_or_else(|| PathBuf::from("."));

    // Keep temporary stream pairs in an application cache directory.
    let temp_base = dirs::cache_dir()
        .map(|p| p.join("Charlie MJ Video Downloader").join("temp"))
        .unwrap_or_else(|| base.join(".charlie-mj-temp"));
    let temp = downloader::temp_pair_dir(&temp_base, &job_id);


    // --------------------------------------------------------
    // Determine Final Output Path
    // --------------------------------------------------------
    // If the frontend supplied an output path, use it.
    //
    // Otherwise, create an MP4 filename using the captured
    // media title.
    let final_path = output_path
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            base.join(format!(
                "{}.mp4",
                sanitize_filename(&title)
            ))
        });


    // --------------------------------------------------------
    // Start Background Download Task
    // --------------------------------------------------------
    // The actual download and FFmpeg processing runs in a
    // background Tokio task.
    tokio::spawn(async move {
        let state = app.state::<AppState>();

        // Temporary file used for the video stream.
        let video_path = temp.join("video.tmp");

        // Temporary file used for the audio stream.
        let audio_path = temp.join("audio.tmp");


        // ----------------------------------------------------
        // Execute Download + Mux + Verification Pipeline
        // ----------------------------------------------------
        set_job_status(state.inner(), &job_id, "downloading", 0.0, None, None);
        let result = async {

            // Download the selected video stream.
            downloader::download_stream(
                &video.url,
                &video_path,
                capture.page_url.as_deref()
            ).await?;


            // If a separate audio stream exists, download it.
            if let Some(a) = &audio {
                downloader::download_stream(
                    &a.url,
                    &audio_path,
                    capture.page_url.as_deref()
                ).await?;
            }


            // ------------------------------------------------
            // Create Output Directory
            // ------------------------------------------------
            // If the final output path contains a parent
            // directory, make sure that directory exists.
            if let Some(parent) = final_path.parent() {
                tokio::fs::create_dir_all(parent)
                    .await
                    .map_err(|e| e.to_string())?;
            }


            // ------------------------------------------------
            // Combine Video and Audio
            // ------------------------------------------------
            // FFmpeg combines the downloaded video and optional
            // audio streams into the final output file.
            ffmpeg::mux(
                &video_path,
                audio.as_ref().map(|_| audio_path.as_path()),
                &final_path
            ).await?;


            // ------------------------------------------------
            // Verify Final Media File
            // ------------------------------------------------
            // Check that the generated output file is valid.
            ffmpeg::verify(&final_path).await?;


            // ------------------------------------------------
            // Remove Temporary Files
            // ------------------------------------------------
            // The video/audio temporary directory is no longer
            // required after successful muxing and verification.
            let _ = tokio::fs::remove_dir_all(&temp).await;


            // Indicate that the complete operation succeeded.
            Ok::<(), String>(())
        }.await;


        // ----------------------------------------------------
        // Cleanup After Failure
        // ----------------------------------------------------
        if let Err(error) = result {
            let _ = tokio::fs::remove_dir_all(&temp).await;
            set_job_status(
                state.inner(),
                &job_id,
                "failed",
                0.0,
                None,
                Some(error)
            );
        } else {
            set_job_status(
                state.inner(),
                &job_id,
                "completed",
                100.0,
                Some(final_path.to_string_lossy().to_string()),
                None
            );
        }
    });

    // Return the job ID to the frontend immediately.
    Ok(job_id)
}


// ============================================================
// List Downloads
// ============================================================
// Purpose:
// - Returns all download jobs currently stored in AppState.
// - Used by the frontend to display download history/status.
// ============================================================

#[tauri::command]
pub fn list_downloads(
    state: State<'_, AppState>
) -> Result<Vec<DownloadItem>, String> {

    // Lock the download state and return a cloned snapshot.
    Ok(
        state.downloads
            .lock()
            .map_err(|_| "state lock failed")?
            .clone()
    )
}


// ============================================================
// List Browser Captures
// ============================================================
// Purpose:
// - Returns browser-detected media captures.
// - Used by the frontend to display captured video/audio
//   streams detected by the Chrome extension.
// ============================================================

#[tauri::command]
pub fn list_browser_captures(
    state: State<'_, AppState>
) -> Result<Vec<BrowserMediaCapture>, String> {

    // Lock the capture state and return a cloned snapshot.
    Ok(
        state.captures
            .lock()
            .map_err(|_| "state lock failed")?
            .clone()
    )
}


// ============================================================
// Cancel Download
// ============================================================
// Purpose:
// - Marks a download as cancelled.
//
// Note:
// - This currently changes the stored status.
// - It does not directly terminate the underlying process.
// ============================================================

#[tauri::command]
pub fn cancel_download(
    id: String,
    state: State<'_, AppState>
) -> Result<(), String> {

    // Find the download using its unique ID.
    if let Some(x) = state.downloads
        .lock()
        .map_err(|_| "state lock failed")?
        .iter_mut()
        .find(|x| x.id == id)
    {
        // Update the download status.
        x.status = "cancelled".into();
    }

    Ok(())
}


// ============================================================
// Pause Download
// ============================================================
// Purpose:
// - Marks a download as paused.
// ============================================================

#[tauri::command]
pub fn pause_download(
    id: String,
    state: State<'_, AppState>
) -> Result<(), String> {

    // Find the requested download.
    if let Some(x) = state.downloads
        .lock()
        .map_err(|_| "state lock failed")?
        .iter_mut()
        .find(|x| x.id == id)
    {
        // Update its status to paused.
        x.status = "paused".into();
    }

    Ok(())
}


// ============================================================
// Resume Download
// ============================================================
// Purpose:
// - Changes a paused download back to the downloading state.
// ============================================================

#[tauri::command]
pub fn resume_download(
    id: String,
    state: State<'_, AppState>
) -> Result<(), String> {

    // Find the requested download.
    if let Some(x) = state.downloads
        .lock()
        .map_err(|_| "state lock failed")?
        .iter_mut()
        .find(|x| x.id == id)
    {
        // Update its status to downloading.
        x.status = "downloading".into();
    }

    Ok(())
}


// ============================================================
// Enable / Disable Browser Watching
// ============================================================
// Purpose:
// - Controls whether browser media watching is enabled.
// - The frontend can use this setting to turn browser capture
//   functionality on or off.
// ============================================================

#[tauri::command]
pub fn set_browser_watch(
    enabled: bool,
    state: State<'_, AppState>
) -> Result<(), String> {

    // Update the shared browser-watch setting.
    *state.browser_watch
        .lock()
        .map_err(|_| "state lock failed")? = enabled;

    Ok(())
}


// ============================================================
// Update Download Job
// ============================================================
fn set_job_status(
    state: &AppState,
    id: &str,
    status: &str,
    progress: f64,
    output_path: Option<String>,
    error: Option<String>,
) {
    if let Ok(mut downloads) = state.downloads.lock() {
        if let Some(job) = downloads.iter_mut().find(|job| job.id == id) {
            job.status = status.to_string();
            job.progress = progress.clamp(0.0, 100.0);
            if let Some(path) = output_path {
                job.output_path = Some(path);
            }
            job.error = error;
            if status == "completed" {
                job.speed_bytes_per_second = 0;
            }
        }
    }
}


// ============================================================
// Create Download Job
// ============================================================
// Purpose:
// - Creates a new DownloadItem.
// - Generates a unique download ID.
// - Adds the job to the shared application state.
// - Initializes progress and download statistics.
//
// This is an internal helper function and is not directly
// exposed to the frontend as a Tauri command.
// ============================================================

fn create_job(
    url: &str,
    title: &str,
    state: &State<'_, AppState>
) -> Result<String, String> {

    // Generate a unique ID for the new download.
    let id = Uuid::new_v4().to_string();


    // Add the new download item to shared application state.
    state.downloads
        .lock()
        .map_err(|_| "state lock failed")?
        .push(DownloadItem {

            // Unique identifier for this download.
            id: id.clone(),

            // Source media URL.
            url: url.into(),

            // Display title / filename information.
            title: title.into(),

            // New downloads begin in the queued state.
            status: "queued".into(),

            // Initial download progress.
            progress: 0.0,

            // No speed is available yet.
            speed_bytes_per_second: 0,

            // Total file size may not be known yet.
            total_bytes: None,

            // No bytes have been downloaded yet.
            downloaded_bytes: 0,

            // Output file path will be assigned later.
            output_path: None,

            // No error has occurred when the job is created.
            error: None,

            // Record the creation time in UTC using RFC3339.
            created_at: Utc::now().to_rfc3339(),
        });


    // Return the newly created job ID.
    Ok(id)
}


// ============================================================
// Sanitize Filename
// ============================================================
// Purpose:
// - Converts a media title into a safer filename.
// - Replaces Windows-invalid characters with "_"
// - Removes unnecessary trailing spaces and periods.
// - Provides a default filename if the result is empty.
//
// Windows-invalid filename characters handled here:
//
// \ / : * ? " < > |
// ============================================================

fn sanitize_filename(input: &str) -> String {

    // Replace invalid filename characters and control
    // characters with an underscore.
    let mut result = input
        .chars()
        .map(|c| {
            if r#"\/:*?"<>|"#.contains(c) || c.is_control() {
                '_'
            } else {
                c
            }
        })
        .collect::<String>();


    // Remove leading/trailing whitespace and trailing periods.
    result = result
        .trim()
        .trim_end_matches('.')
        .to_string();


    // If nothing remains, use a safe default filename.
    if result.is_empty() {
        "Charlie MJ Video".into()
    } else {
        result
    }
}


// ============================================================
// Validate URL
// ============================================================
// Purpose:
// - Ensures that the supplied URL is valid.
// - Only HTTP and HTTPS URLs are accepted.
//
// This prevents unsupported URL schemes from being passed
// to the media/download components.
// ============================================================

fn validate_url(value: &str) -> Result<(), String> {

    // Parse the supplied string as a URL.
    let parsed = url::Url::parse(value)
        .map_err(|_| "Invalid URL".to_string())?;


    // Accept only HTTP and HTTPS URLs.
    match parsed.scheme() {

        // Supported web protocols.
        "http" | "https" => Ok(()),

        // Reject all other protocols.
        _ => Err(
            "Only HTTP and HTTPS URLs are supported".into()
        ),
    }
}