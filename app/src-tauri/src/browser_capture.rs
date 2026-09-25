// ============================================================
// Charlie MJ Video Downloader - Browser Media Capture
// ============================================================
// Purpose:
// - Receive media information captured by the Chrome extension
// - Validate and classify video/audio streams
// - Normalize Google video playback URLs
// - Convert incoming JSON data into Rust structures
// - Remove duplicate media streams
// - Create unique capture and stream IDs
//
// Data flow:
//
// Chrome Extension
//       ↓
// Native Messaging Host
//       ↓
// JSON message
//       ↓
// parse_message()
//       ↓
// BrowserMediaCapture
//       ↓
// Charlie MJ Rust Backend
// ============================================================


// ------------------------------------------------------------
// Import Application Models
// ------------------------------------------------------------
// BrowserMediaCapture:
//   Represents a complete media detection/capture event.
//
// CapturedStream:
//   Represents an individual detected video or audio stream.
use crate::models::{BrowserMediaCapture, CapturedStream};


// ------------------------------------------------------------
// Import Date/Time Support
// ------------------------------------------------------------
// Utc is used to generate the creation timestamp for each
// browser media capture.
use chrono::Utc;


// ------------------------------------------------------------
// Import JSON Value Type
// ------------------------------------------------------------
// serde_json::Value allows this module to read and inspect
// dynamic JSON data received from the browser/native host.
use serde_json::Value;


// ------------------------------------------------------------
// Import HashMap
// ------------------------------------------------------------
// HashMap is used to temporarily store streams and remove
// duplicate entries before creating BrowserMediaCapture.
use std::collections::HashMap;


// ------------------------------------------------------------
// Import UUID Generator
// ------------------------------------------------------------
// UUIDs are used to generate unique identifiers for captures
// and individual captured streams.
use uuid::Uuid;


// ============================================================
// Classify MIME Type
// ============================================================
// Determines whether a MIME/content type represents:
//
// - video
// - audio
// - unknown
//
// Example:
//
// "video/mp4"          → video
// "video/webm"         → video
// "audio/mpeg"         → audio
// "audio/mp4"          → audio
// "text/html"          → unknown
//
// The MIME value may contain additional parameters such as:
//
// "video/mp4; codecs=avc1"
//
// Only the main MIME type before the semicolon is checked.
fn classify_mime(mime: Option<&str>) -> &'static str {

    // Use an empty string when no MIME type was provided.
    //
    // split(';'):
    //   Removes optional MIME parameters.
    //
    // next():
    //   Gets the main MIME type.
    match mime
        .unwrap_or_default()
        .split(';')
        .next()
        .unwrap_or_default()
    {

        // MIME type starts with "video/".
        x if x.starts_with("video/") => "video",

        // MIME type starts with "audio/".
        x if x.starts_with("audio/") => "audio",

        // Anything else is not recognized as supported media.
        _ => "unknown",
    }
}


// ============================================================
// Normalize Google Drive / Google Video URL
// ============================================================
// Cleans temporary query parameters from Google video playback
// URLs.
//
// Some Google video URLs contain request-specific parameters
// that can change even though the underlying media stream is
// the same.
//
// Removing these parameters helps prevent duplicate streams
// from being stored.
//
// If the URL cannot be parsed, the original URL is returned.
fn normalized_drive_url(raw: &str) -> String {
    // Preserve the exact browser-issued URL. Google video playback
    // URLs can contain signed/temporary parameters that are required
    // for authorization.
    raw.to_string()
}


// ============================================================
// Parse Browser Media Detection Message
// ============================================================
// Converts a JSON message received from the browser/native
// messaging layer into a strongly structured
// BrowserMediaCapture object.
//
// Expected message type:
//
// {
//   "type": "media_detected",
//   "source": "...",
//   "pageUrl": "...",
//   "title": "...",
//   "streams": [...]
//
// }
//
// Returns:
//
// Ok(BrowserMediaCapture)
//     → Message was successfully parsed.
//
// Err(String)
//     → Message was invalid or unsupported.
pub fn parse_message(value: Value) -> Result<BrowserMediaCapture, String> {

    // ----------------------------------------------------------
    // Validate Message Type
    // ----------------------------------------------------------
    // Only "media_detected" messages are accepted by this
    // parser.
    //
    // Other message types such as "open_url" are handled
    // elsewhere.
    if value.get("type").and_then(Value::as_str) != Some("media_detected") {
        return Err("Unsupported native message type".into());
    }


    // ----------------------------------------------------------
    // Read Capture Metadata
    // ----------------------------------------------------------
    // Get the source of the detected media.
    //
    // If source is missing, default to "browser".
    let source = value
        .get("source")
        .and_then(Value::as_str)
        .unwrap_or("browser")
        .to_string();


    // Get the URL of the webpage where the media was detected.
    //
    // The value is optional.
    let page_url = value
        .get("pageUrl")
        .and_then(Value::as_str)
        .map(str::to_string);


    // Get the webpage title.
    //
    // The value is optional.
    let title = value
        .get("title")
        .and_then(Value::as_str)
        .map(str::to_string);


    // ----------------------------------------------------------
    // Create Temporary Stream Collection
    // ----------------------------------------------------------
    // A HashMap is used for deduplication.
    //
    // Key:
    //   Combination of stream type, URL, width and height.
    //
    // Value:
    //   CapturedStream object.
    //
    // This prevents identical media streams from appearing
    // multiple times in the final capture.
    let mut dedupe: HashMap<String, CapturedStream> = HashMap::new();


    // ----------------------------------------------------------
    // Read Streams Array
    // ----------------------------------------------------------
    // Get the "streams" array from the incoming JSON message.
    //
    // If the field does not exist, use an empty array instead.
    for item in value
        .get("streams")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default()
    {

        // ------------------------------------------------------
        // Extract Stream URL
        // ------------------------------------------------------
        // Every supported stream must have a URL.
        //
        // If the URL is missing, the entire message parsing
        // operation returns an error.
        let raw_url = item
            .get("url")
            .and_then(Value::as_str)
            .ok_or("Stream URL missing")?;


        // Normalize the URL before storing it.
        let url = normalized_drive_url(raw_url);


        // ------------------------------------------------------
        // Extract MIME Type
        // ------------------------------------------------------
        // MIME is optional because the browser extension may
        // not always provide a Content-Type value.
        let mime = item
            .get("mime")
            .and_then(Value::as_str)
            .map(str::to_string);


        // ------------------------------------------------------
        // Determine Stream Type
        // ------------------------------------------------------
        // Prefer the explicit "streamType" field.
        //
        // If streamType is not available, attempt to determine
        // the type from the MIME value.
        let stream_type = item
            .get("streamType")
            .and_then(Value::as_str)
            .unwrap_or_else(|| classify_mime(mime.as_deref()))
            .to_string();


        // ------------------------------------------------------
        // Ignore Unsupported Stream Types
        // ------------------------------------------------------
        // Only video and audio streams are relevant to the
        // Charlie MJ downloader.
        //
        // Other types are ignored instead of causing the entire
        // message to fail.
        if !matches!(stream_type.as_str(), "video" | "audio") {
            continue;
        }


        // ------------------------------------------------------
        // Generate Unique Stream ID
        // ------------------------------------------------------
        // A UUID provides a unique identifier for the captured
        // stream.
        let id = Uuid::new_v4().to_string();


        // ------------------------------------------------------
        // Create CapturedStream
        // ------------------------------------------------------
        // Convert the JSON stream information into the Rust
        // CapturedStream structure.
        let stream = CapturedStream {

            // Unique stream identifier.
            id: id.clone(),

            // Source of the stream, such as browser or
            // google_drive.
            source: source.clone(),

            // Browser tab ID, if supplied.
            tab_id: item
                .get("tabId")
                .and_then(Value::as_i64),

            // Page URL where the stream was detected.
            page_url: page_url.clone(),

            // Page title where the stream was detected.
            title: title.clone(),

            // Stream type:
            // "video" or "audio".
            stream_type,

            // Normalized media URL.
            url,

            // MIME/content type, if available.
            mime,

            // Quality information, if supplied.
            quality: item
                .get("quality")
                .and_then(Value::as_str)
                .map(str::to_string),

            // Video width in pixels.
            width: item
                .get("width")
                .and_then(Value::as_u64)
                .map(|x| x as u32),

            // Video height in pixels.
            height: item
                .get("height")
                .and_then(Value::as_u64)
                .map(|x| x as u32),

            // Frames per second.
            fps: item
                .get("fps")
                .and_then(Value::as_f64),

            // Bitrate information.
            bitrate: item
                .get("bitrate")
                .and_then(Value::as_u64),

            // Stream/file size in bytes.
            size: item
                .get("size")
                .and_then(Value::as_u64),

            // Media duration in seconds.
            duration: item
                .get("duration")
                .and_then(Value::as_f64),
        };


        // ------------------------------------------------------
        // Build Deduplication Key
        // ------------------------------------------------------
        // Two streams are considered duplicates when they have
        // the same:
        //
        // - stream type
        // - URL
        // - width
        // - height
        //
        // The {:?} formatting safely represents optional values
        // such as Some(1920) or None.
        let key = format!(
            "{}|{}|{:?}|{:?}",
            stream.stream_type,
            stream.url,
            stream.width,
            stream.height
        );


        // ------------------------------------------------------
        // Store Only the First Matching Stream
        // ------------------------------------------------------
        // If the key already exists, keep the existing stream.
        //
        // If the key does not exist, insert this stream.
        dedupe.entry(key).or_insert(stream);
    }


    // ========================================================
    // Create Final BrowserMediaCapture
    // ========================================================
    // Once all streams have been processed, create the final
    // browser capture object.
    Ok(BrowserMediaCapture {

        // Generate a unique ID for the entire capture event.
        capture_id: Uuid::new_v4().to_string(),

        // Source of the capture.
        source,

        // URL of the webpage where media was detected.
        page_url,

        // Title of the webpage.
        title,

        // Convert the deduplicated HashMap into a Vec.
        streams: dedupe.into_values().collect(),

        // Record the UTC creation timestamp in RFC3339 format.
        //
        // Example:
        // 2026-09-25T05:19:00+00:00
        created_at: Utc::now().to_rfc3339(),
    })
}