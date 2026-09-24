use crate::models::{MediaFormat, MediaInfo};
use serde_json::Value;
use std::{path::PathBuf, process::Stdio};
use tokio::process::Command;

fn yt_dlp_path() -> PathBuf {
    if let Ok(path) = std::env::var("CHARLIE_MJ_YTDLP") {
        return PathBuf::from(path);
    }

    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|x| x.to_path_buf()))
        .unwrap_or_default();

    exe_dir.join("tools").join("yt-dlp.exe")
}

pub async fn analyze(url: &str) -> Result<MediaInfo, String> {
    let output = Command::new(yt_dlp_path())
        .arg("--dump-single-json")
        .arg("--no-playlist")
        .arg("--skip-download")
        .arg(url)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| format!("Unable to start yt-dlp: {e}"))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }

    let raw: Value = serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("Invalid yt-dlp JSON: {e}"))?;

    let formats = raw["formats"]
        .as_array()
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .map(|f| MediaFormat {
            id: f["format_id"].as_str().unwrap_or_default().to_string(),
            extension: f["ext"].as_str().unwrap_or_default().to_string(),
            resolution: f["resolution"].as_str().map(str::to_string),
            fps: f["fps"].as_f64(),
            filesize: f["filesize"].as_u64().or_else(|| f["filesize_approx"].as_u64()),
            vcodec: f["vcodec"].as_str().map(str::to_string),
            acodec: f["acodec"].as_str().map(str::to_string),
            abr: f["abr"].as_f64(),
            note: f["format_note"].as_str().map(str::to_string),
        })
        .collect();

    let subtitles = raw["subtitles"]
        .as_object()
        .map(|m| m.keys().cloned().collect())
        .unwrap_or_default();

    Ok(MediaInfo {
        id: raw["id"].as_str().unwrap_or_default().to_string(),
        title: raw["title"].as_str().unwrap_or("Untitled").to_string(),
        uploader: raw["uploader"].as_str().map(str::to_string),
        duration: raw["duration"].as_f64(),
        thumbnail: raw["thumbnail"].as_str().map(str::to_string),
        webpage_url: raw["webpage_url"].as_str().unwrap_or(url).to_string(),
        formats,
        subtitles,
    })
}

pub async fn download(url: &str, format_id: Option<&str>, output_template: Option<&str>) -> Result<(), String> {
    let mut command = Command::new(yt_dlp_path());
    command.arg("--newline").arg("--no-playlist");

    match format_id {
        Some("bestaudio") => {
            command.args(["-f", "bestaudio/best", "-x", "--audio-format", "mp3"]);
        }
        Some(id) => {
            command.args(["-f", id]);
        }
        None => {
            command.args(["-f", "bestvideo*+bestaudio/best", "--merge-output-format", "mp4"]);
        }
    }

    command.arg("-o").arg(output_template.unwrap_or("%(title)s.%(ext)s"));
    command.arg(url);

    let output = command.output().await.map_err(|e| e.to_string())?;
    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}
