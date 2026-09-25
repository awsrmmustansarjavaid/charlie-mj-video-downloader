use std::{path::{Path, PathBuf}, process::Stdio};
use tokio::process::Command;

fn tool(name: &str) -> PathBuf {
    if let Ok(dir) = std::env::var("CHARLIE_MJ_TOOLS_DIR") {
        return PathBuf::from(dir).join(name);
    }
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|x| x.to_path_buf()))
        .unwrap_or_default()
        .join("tools")
        .join(name)
}

pub async fn mux(video: &Path, audio: Option<&Path>, output: &Path) -> Result<(), String> {
    let mut copy = Command::new(tool("ffmpeg.exe"));
    copy.args(["-hide_banner", "-loglevel", "error", "-y"]);
    copy.args(["-i", video.to_string_lossy().as_ref()]);
    if let Some(audio) = audio {
        copy.args(["-i", audio.to_string_lossy().as_ref()]);
        copy.args(["-map", "0:v:0", "-map", "1:a:0"]);
    } else {
        copy.args(["-map", "0"]);
    }
    copy.args(["-c", "copy", output.to_string_lossy().as_ref()]);
    let first = copy.stdout(Stdio::null()).stderr(Stdio::piped()).output().await
        .map_err(|e| format!("FFmpeg start failed: {e}"))?;

    if first.status.success() {
        return Ok(());
    }

    let mut fallback = Command::new(tool("ffmpeg.exe"));
    fallback.args(["-hide_banner", "-loglevel", "error", "-y"]);
    fallback.args(["-i", video.to_string_lossy().as_ref()]);
    if let Some(audio) = audio {
        fallback.args(["-i", audio.to_string_lossy().as_ref()]);
        fallback.args(["-map", "0:v:0", "-map", "1:a:0"]);
    }
    fallback.args([
        "-c:v", "libx264",
        "-preset", "medium",
        "-crf", "20",
        "-c:a", "aac",
        "-b:a", "192k",
        output.to_string_lossy().as_ref()
    ]);

    let second = fallback.output().await.map_err(|e| e.to_string())?;
    if second.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&second.stderr).to_string())
    }
}

pub async fn verify(path: &Path) -> Result<(), String> {
    let output = Command::new(tool("ffprobe.exe"))
        .args([
            "-v", "error",
            "-show_entries", "format=duration,size",
            "-show_streams",
            "-of", "json",
            path.to_string_lossy().as_ref()
        ])
        .output()
        .await
        .map_err(|e| e.to_string())?;

    if !output.status.success() || output.stdout.is_empty() {
        return Err("ffprobe could not verify the final media file".into());
    }

    Ok(())
}
