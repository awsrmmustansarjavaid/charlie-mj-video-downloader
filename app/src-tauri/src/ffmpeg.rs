// ============================================================
// Charlie MJ Video Downloader - FFmpeg Integration
// ============================================================
// Purpose:
// - Locates FFmpeg and FFprobe executables.
// - Combines separate video and audio streams.
// - Uses stream copying when possible for fast processing.
// - Falls back to video/audio re-encoding when necessary.
// - Verifies the final media file.
//
// Main tools:
//
// ffmpeg.exe
//   → Muxes or re-encodes video/audio.
//
// ffprobe.exe
//   → Inspects and verifies the final media file.
//
// Typical flow:
//
// video.tmp + audio.tmp
//        ↓
//     FFmpeg
//        ↓
//    final.mp4
//        ↓
//    FFprobe
//        ↓
//    Verified Media
// ============================================================

use std::{
    // Path is used for input/output file locations.
    // PathBuf is used when constructing executable paths.
    path::{Path, PathBuf},

    // Stdio is used to control FFmpeg process output.
    process::Stdio,
};

use tokio::process::Command;


// ============================================================
// Locate External Tool
// ============================================================
// Purpose:
// - Determines where FFmpeg or FFprobe is installed.
//
// Tool lookup order:
//
// 1. CHARLIE_MJ_TOOLS_DIR environment variable
// 2. Directory next to the application executable
//    followed by "tools"
//
// Example:
//
// CHARLIE_MJ_TOOLS_DIR
//     ↓
// C:\CharlieMJ\tools\ffmpeg.exe
//
// Without the environment variable:
//
// Application directory
//     ↓
// tools
//     ↓
// ffmpeg.exe / ffprobe.exe
// ============================================================

fn tool(name: &str) -> PathBuf {
    if let Ok(dir) = std::env::var("CHARLIE_MJ_TOOLS_DIR") {
        let candidate = PathBuf::from(dir).join(name);
        if candidate.exists() {
            return candidate;
        }
    }

    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(PathBuf::from))
        .unwrap_or_default();

    let candidates = [
        exe_dir.join("resources").join("tools").join(name),
        exe_dir.join("tools").join(name),
        exe_dir.join("..\\resources\\tools").join(name),
    ];

    candidates
        .into_iter()
        .find(|p| p.exists())
        .unwrap_or_else(|| exe_dir.join("resources").join("tools").join(name))
}


// ============================================================
// Mux Video and Audio
// ============================================================
// Purpose:
// - Combines a video file and optional audio file.
// - First attempts stream copying for fast processing.
// - If stream copying fails, performs a fallback re-encode.
//
// Parameters:
// - video  → Input video file.
// - audio  → Optional separate audio file.
// - output → Final media file.
//
// Strategy:
//
// Attempt 1:
//     -c copy
//     ↓
// No re-encoding
//     ↓
// Fast and preserves original quality
//
// If that fails:
//
// Attempt 2:
//     H.264 video + AAC audio
//     ↓
// Compatible MP4 output
// ============================================================

pub async fn mux(
    video: &Path,
    audio: Option<&Path>,
    output: &Path
) -> Result<(), String> {

    // --------------------------------------------------------
    // First FFmpeg Attempt: Stream Copy
    // --------------------------------------------------------
    // Create an FFmpeg process using the configured
    // ffmpeg.exe executable.
    let mut copy = Command::new(tool("ffmpeg.exe"));


    // Hide the FFmpeg startup banner and show only errors.
    //
    // -y automatically overwrites the output file if it exists.
    copy.args([
        "-hide_banner",
        "-loglevel", "error",
        "-y"
    ]);


    // Add the video input file.
    copy.args([
        "-i",
        video.to_string_lossy().as_ref()
    ]);


    // --------------------------------------------------------
    // Add Optional Audio Input
    // --------------------------------------------------------
    // If a separate audio stream exists, add it as the second
    // FFmpeg input.
    if let Some(audio) = audio {

        copy.args([
            "-i",
            audio.to_string_lossy().as_ref()
        ]);


        // Select:
        // - Video stream from input 0
        // - Audio stream from input 1
        copy.args([
            "-map", "0:v:0",
            "-map", "1:a:0"
        ]);

    } else {

        // If there is no separate audio input, use all streams
        // from the video input.
        copy.args([
            "-map", "0"
        ]);
    }


    // --------------------------------------------------------
    // Copy Existing Encoded Streams
    // --------------------------------------------------------
    // "-c copy" tells FFmpeg not to re-encode the media.
    //
    // Advantages:
    // - Much faster processing.
    // - No additional quality loss.
    // - Lower CPU usage.
    copy.args([
        "-c", "copy",
        output.to_string_lossy().as_ref()
    ]);


    // --------------------------------------------------------
    // Execute First FFmpeg Process
    // --------------------------------------------------------
    // stdout is discarded because only errors are important.
    // stderr is captured so it can be inspected if the command
    // fails.
    let first = copy
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| format!("FFmpeg start failed: {e}"))?;


    // --------------------------------------------------------
    // Check Stream-Copy Result
    // --------------------------------------------------------
    // If FFmpeg successfully combined the streams without
    // re-encoding, the job is complete.
    if first.status.success() {
        return Ok(());
    }


    // ========================================================
    // Fallback FFmpeg Process
    // ========================================================
    // Stream copying can fail when the input codecs/container
    // combination is not compatible with the requested output.
    //
    // In that situation, create a second FFmpeg command that
    // re-encodes the media into H.264/AAC.
    let mut fallback = Command::new(tool("ffmpeg.exe"));


    // Hide the banner, show errors, and overwrite the output.
    fallback.args([
        "-hide_banner",
        "-loglevel", "error",
        "-y"
    ]);


    // Add the video input.
    fallback.args([
        "-i",
        video.to_string_lossy().as_ref()
    ]);


    // --------------------------------------------------------
    // Add Optional Audio to Fallback
    // --------------------------------------------------------
    if let Some(audio) = audio {

        // Add the separate audio file.
        fallback.args([
            "-i",
            audio.to_string_lossy().as_ref()
        ]);


        // Select video from the first input and audio from the
        // second input.
        fallback.args([
            "-map", "0:v:0",
            "-map", "1:a:0"
        ]);
    }


    // --------------------------------------------------------
    // Fallback Encoding Settings
    // --------------------------------------------------------
    // Video:
    // - libx264 → H.264 video encoder
    // - medium  → Encoding speed/quality preset
    // - CRF 20  → Quality target
    //
    // Audio:
    // - AAC encoder
    // - 192 kbps bitrate
    //
    // This creates a broadly compatible MP4 file.
    fallback.args([
        "-c:v", "libx264",
        "-preset", "medium",
        "-crf", "20",
        "-c:a", "aac",
        "-b:a", "192k",

        // Final output path.
        output.to_string_lossy().as_ref()
    ]);


    // --------------------------------------------------------
    // Execute Fallback FFmpeg Process
    // --------------------------------------------------------
    let second = fallback
        .output()
        .await
        .map_err(|e| e.to_string())?;


    // --------------------------------------------------------
    // Check Fallback Result
    // --------------------------------------------------------
    if second.status.success() {

        // Re-encoding completed successfully.
        Ok(())

    } else {

        // Return FFmpeg's error output so the caller can see
        // why the media processing failed.
        Err(
            String::from_utf8_lossy(&second.stderr).to_string()
        )
    }
}


// ============================================================
// Verify Final Media File
// ============================================================
// Purpose:
// - Uses FFprobe to inspect the generated media file.
// - Checks that FFprobe can successfully read the file.
// - Requests duration, size, and stream information.
//
// Parameters:
// - path → Final media file to verify.
//
// Returns:
// - Ok(()) when FFprobe successfully reads the file.
// - Err(String) when verification fails.
// ============================================================

pub async fn verify(path: &Path) -> Result<(), String> {

    // --------------------------------------------------------
    // Run FFprobe
    // --------------------------------------------------------
    let output = Command::new(tool("ffprobe.exe"))
        .args([

            // Show only errors.
            "-v", "error",

            // Request basic format information such as:
            // - duration
            // - file size
            "-show_entries", "format=duration,size",

            // Request information about media streams.
            "-show_streams",

            // Return the information as JSON.
            "-of", "json",

            // File that should be inspected.
            path.to_string_lossy().as_ref()
        ])
        .output()
        .await
        .map_err(|e| e.to_string())?;


    // --------------------------------------------------------
    // Validate FFprobe Result
    // --------------------------------------------------------
    // A successful verification requires:
    //
    // 1. FFprobe process exited successfully.
    // 2. FFprobe produced output.
    if !output.status.success() || output.stdout.is_empty() {

        return Err(
            "ffprobe could not verify the final media file".into()
        );
    }


    // Verification completed successfully.
    Ok(())
}