use crate::{
    media,
    models::DownloadItem,
    state::AppState,
};
use chrono::Utc;
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

    let id = Uuid::new_v4().to_string();
    let item = DownloadItem {
        id: id.clone(),
        url: url.clone(),
        title: url.clone(),
        status: "downloading".into(),
        progress: 0.0,
        speed_bytes_per_second: 0,
        total_bytes: None,
        downloaded_bytes: 0,
        output_path: None,
        error: None,
        created_at: Utc::now().to_rfc3339(),
    };

    state.downloads.lock().map_err(|_| "state lock failed")?.push(item);

    tauri::async_runtime::spawn(async move {
        let _ = media::download(&url, format_id.as_deref(), output_template.as_deref()).await;
    });

    Ok(id)
}

#[tauri::command]
pub fn list_downloads(state: State<'_, AppState>) -> Result<Vec<DownloadItem>, String> {
    Ok(state.downloads.lock().map_err(|_| "state lock failed")?.clone())
}

#[tauri::command]
pub fn cancel_download(id: String, state: State<'_, AppState>) -> Result<(), String> {
    if let Some(item) = state.downloads.lock().map_err(|_| "state lock failed")?.iter_mut().find(|x| x.id == id) {
        item.status = "cancelled".into();
    }
    Ok(())
}

#[tauri::command]
pub fn pause_download(id: String, state: State<'_, AppState>) -> Result<(), String> {
    if let Some(item) = state.downloads.lock().map_err(|_| "state lock failed")?.iter_mut().find(|x| x.id == id) {
        item.status = "paused".into();
    }
    Ok(())
}

#[tauri::command]
pub fn resume_download(id: String, state: State<'_, AppState>) -> Result<(), String> {
    if let Some(item) = state.downloads.lock().map_err(|_| "state lock failed")?.iter_mut().find(|x| x.id == id) {
        item.status = "downloading".into();
    }
    Ok(())
}

fn validate_url(value: &str) -> Result<(), String> {
    let parsed = url::Url::parse(value).map_err(|_| "Invalid URL".to_string())?;
    match parsed.scheme() {
        "http" | "https" => Ok(()),
        _ => Err("Only HTTP and HTTPS URLs are supported".into()),
    }
}
