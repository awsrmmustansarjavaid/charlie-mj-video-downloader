// ============================================================
// Charlie MJ Video Downloader - Frontend Type Definitions
// ============================================================
// Purpose:
// - Defines the TypeScript data models used by the React UI.
// - Keeps frontend data structures consistent with the Rust
//   backend models.
// - Provides type safety when working with media, downloads,
//   and browser-captured streams.
//
// Data flow:
//
// Rust Backend
//     ↓
// JSON / Tauri API
//     ↓
// services/api.ts
//     ↓
// types.ts
//     ↓
// App.tsx
// ============================================================


// ============================================================
// Download Status
// ============================================================
// Represents the possible states of a download job.
//
// queued      → Job created and waiting to start
// analyzing   → Media information is being analyzed
// downloading → Media is currently being downloaded
// processing  → Downloaded media is being processed/combined
// paused      → Download has been paused
// completed   → Download finished successfully
// failed      → Download failed
// cancelled   → Download was cancelled
// ============================================================
export type DownloadStatus =
  | "queued"
  | "analyzing"
  | "downloading"
  | "processing"
  | "paused"
  | "completed"
  | "failed"
  | "cancelled";


// ============================================================
// Media Format
// ============================================================
// Describes one available media format returned by the
// backend media analyzer.
//
// A media item can contain many formats with different:
// - Resolutions
// - Frame rates
// - File sizes
// - Video codecs
// - Audio codecs
// ============================================================
export interface MediaFormat {

  // Unique format identifier returned by the media analyzer.
  id: string;

  // File/container extension, such as mp4, webm, or m4a.
  extension: string;

  // Video resolution, when available.
  // Example: "1920x1080"
  resolution?: string;

  // Video frame rate in frames per second.
  fps?: number;

  // Estimated or known file size in bytes.
  filesize?: number;

  // Video codec used by the format.
  // Example: avc1, vp9, av01
  vcodec?: string;

  // Audio codec used by the format.
  // Example: aac, opus, mp4a
  acodec?: string;

  // Audio bitrate in kilobits per second.
  abr?: number;

  // Additional information about the format.
  note?: string;
}


// ============================================================
// Media Information
// ============================================================
// Contains the metadata returned after analyzing a media URL.
//
// This information is displayed by App.tsx before the user
// selects a download format.
// ============================================================
export interface MediaInfo {

  // Unique media identifier.
  id: string;

  // Media title.
  title: string;

  // Uploader/channel/creator name, when available.
  uploader?: string;

  // Media duration in seconds.
  duration?: number;

  // Preview/thumbnail image URL.
  thumbnail?: string;

  // Original webpage URL containing the media.
  webpageUrl: string;

  // All available downloadable media formats.
  formats: MediaFormat[];

  // List of available subtitle languages or identifiers.
  subtitles: string[];
}


// ============================================================
// Captured Stream
// ============================================================
// Represents one media stream detected from the browser.
//
// A browser capture may contain separate:
// - Video stream
// - Audio stream
//
// These streams can later be downloaded separately and combined
// by the Rust backend using FFmpeg.
// ============================================================
export interface CapturedStream {

  // Unique identifier for this captured stream.
  id: string;

  // Source of the captured media.
  // Example: browser, google_drive, etc.
  source: string;

  // Chrome tab ID from which the stream was captured.
  tabId?: number;

  // URL of the browser page where the media was detected.
  pageUrl?: string;

  // Page/media title when available.
  title?: string;

  // Type of captured media stream.
  type: "video" | "audio" | "unknown";

  // Direct media stream URL.
  url: string;

  // MIME type of the stream.
  // Example: video/mp4 or audio/mp4
  mime?: string;

  // Quality information supplied by the browser capture.
  quality?: string;

  // Video width in pixels.
  width?: number;

  // Video height in pixels.
  height?: number;

  // Video frame rate.
  fps?: number;

  // Stream bitrate in bits per second.
  bitrate?: number;

  // Stream size in bytes, when available.
  size?: number;

  // Stream duration in seconds, when available.
  duration?: number;
}


// ============================================================
// Browser Media Capture
// ============================================================
// Represents a complete browser media detection event.
//
// One capture can contain multiple CapturedStream objects,
// such as separate video and audio streams.
// ============================================================
export interface BrowserMediaCapture {

  // Unique identifier for the browser capture.
  captureId: string;

  // Source of the capture.
  source: string;

  // Browser page URL associated with the capture.
  pageUrl?: string;

  // Page/media title when available.
  title?: string;

  // Video and audio streams detected during the capture.
  streams: CapturedStream[];

  // Timestamp indicating when the capture was created.
  createdAt: string;
}


// ============================================================
// Download Item
// ============================================================
// Represents one download job managed by Charlie MJ.
//
// App.tsx uses this structure to display:
// - Download title
// - Current status
// - Progress
// - Download speed
// - Output location
// - Errors
// ============================================================
export interface DownloadItem {

  // Unique identifier for the download job.
  id: string;

  // Source media URL.
  url: string;

  // Display title of the download.
  title: string;

  // Current download state.
  status: DownloadStatus;

  // Download progress percentage.
  // Expected range: 0 - 100.
  progress: number;

  // Current download speed in bytes per second.
  speedBytesPerSecond: number;

  // Total media size in bytes, when known.
  totalBytes?: number;

  // Number of bytes downloaded so far.
  downloadedBytes: number;

  // Final output file path, when available.
  outputPath?: string;

  // Error message when the download fails.
  error?: string;

  // Timestamp indicating when the download job was created.
  createdAt: string;
}