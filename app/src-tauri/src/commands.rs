use crate::{
    downloader, ffmpeg, media,
    models::{BrowserMediaCapture, DownloadItem},
    state::AppState,
};
use chrono::Utc;
use std::path::PathBuf;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub async fn analyze_url(url: String) -> Result<crate::models::MediaInfo, String> {
    validate_url(&url)?;
    media::analyze(&url).await
}

#[tauri::command]
pub async fn start_download(
    url: String,
    format_id: Option<String>,
    output_template: Option<String>,
    state: State<'_, AppState>,
) -> Result<String, String> {
    validate_url(&url)?;
    let id = create_job(&url, &url, &state)?;

    tokio::spawn(async move {
        let _ = media::download(&url, format_id.as_deref(), output_template.as_deref()).await;
    });

    Ok(id)
}

#[tauri::command]
pub async fn start_captured_download(
    capture_id: String,
    video_stream_id: String,
    audio_stream_id: Option<String>,
    output_path: Option<String>,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let capture = state.captures.lock()
        .map_err(|_| "capture state unavailable")?
        .iter()
        .find(|x| x.capture_id == capture_id)
        .cloned()
        .ok_or("Capture no longer exists")?;

    let video = capture.streams.iter()
        .find(|x| x.id == video_stream_id)
        .cloned()
        .ok_or("Video stream not found")?;

    let audio = audio_stream_id.and_then(|id| capture.streams.iter().find(|x| x.id == id).cloned());

    let job_id = Uuid::new_v4().to_string();
    let title = capture.title.clone().unwrap_or_else(|| "Charlie MJ Video".into());
    let base = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let temp = downloader::temp_pair_dir(&base, &job_id);
    let final_path = output_path.map(PathBuf::from)
        .unwrap_or_else(|| base.join(format!("{}.mp4", sanitize_filename(&title))));

    let _ = create_job(&video.url, &title, &state)?;

    tokio::spawn(async move {
        let video_path = temp.join("video.tmp");
        let audio_path = temp.join("audio.tmp");

        let result = async {
            downloader::download_stream(&video.url, &video_path).await?;
            if let Some(a) = &audio {
                downloader::download_stream(&a.url, &audio_path).await?;
            }

            if let Some(parent) = final_path.parent() {
                tokio::fs::create_dir_all(parent).await.map_err(|e| e.to_string())?;
            }

            ffmpeg::mux(
                &video_path,
                audio.as_ref().map(|_| audio_path.as_path()),
                &final_path
            ).await?;

            ffmpeg::verify(&final_path).await?;
            let _ = tokio::fs::remove_dir_all(&temp).await;
            Ok::<(), String>(())
        }.await;

        if result.is_err() {
            let _ = tokio::fs::remove_dir_all(&temp).await;
        }
    });

    Ok(job_id)
}

#[tauri::command]
pub fn list_downloads(state: State<'_, AppState>) -> Result<Vec<DownloadItem>, String> {
    Ok(state.downloads.lock().map_err(|_| "state lock failed")?.clone())
}

#[tauri::command]
pub fn list_browser_captures(state: State<'_, AppState>) -> Result<Vec<BrowserMediaCapture>, String> {
    Ok(state.captures.lock().map_err(|_| "state lock failed")?.clone())
}

#[tauri::command]
pub fn cancel_download(id: String, state: State<'_, AppState>) -> Result<(), String> {
    if let Some(x) = state.downloads.lock().map_err(|_| "state lock failed")?.iter_mut().find(|x| x.id == id) {
        x.status = "cancelled".into();
    }
    Ok(())
}

#[tauri::command]
pub fn pause_download(id: String, state: State<'_, AppState>) -> Result<(), String> {
    if let Some(x) = state.downloads.lock().map_err(|_| "state lock failed")?.iter_mut().find(|x| x.id == id) {
        x.status = "paused".into();
    }
    Ok(())
}

#[tauri::command]
pub fn resume_download(id: String, state: State<'_, AppState>) -> Result<(), String> {
    if let Some(x) = state.downloads.lock().map_err(|_| "state lock failed")?.iter_mut().find(|x| x.id == id) {
        x.status = "downloading".into();
    }
    Ok(())
}

#[tauri::command]
pub fn set_browser_watch(enabled: bool, state: State<'_, AppState>) -> Result<(), String> {
    *state.browser_watch.lock().map_err(|_| "state lock failed")? = enabled;
    Ok(())
}

fn create_job(url: &str, title: &str, state: &State<'_, AppState>) -> Result<String, String> {
    let id = Uuid::new_v4().to_string();
    state.downloads.lock().map_err(|_| "state lock failed")?.push(DownloadItem {
        id: id.clone(),
        url: url.into(),
        title: title.into(),
        status: "queued".into(),
        progress: 0.0,
        speed_bytes_per_second: 0,
        total_bytes: None,
        downloaded_bytes: 0,
        output_path: None,
        error: None,
        created_at: Utc::now().to_rfc3339(),
    });
    Ok(id)
}

fn sanitize_filename(input: &str) -> String {
    let mut result = input.chars().map(|c| {
        if r#"\/:*?"<>|"#.contains(c) || c.is_control() { '_' } else { c }
    }).collect::<String>();
    result = result.trim().trim_end_matches('.').to_string();
    if result.is_empty() { "Charlie MJ Video".into() } else { result }
}

fn validate_url(value: &str) -> Result<(), String> {
    let parsed = url::Url::parse(value).map_err(|_| "Invalid URL".to_string())?;
    match parsed.scheme() {
        "http" | "https" => Ok(()),
        _ => Err("Only HTTP and HTTPS URLs are supported".into()),
    }
}
