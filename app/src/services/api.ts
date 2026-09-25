// ============================================================
// Charlie MJ Video Downloader - Tauri API Service
// ============================================================
// Purpose:
// - Provides a single frontend API layer for communicating
//   with the Rust/Tauri backend.
// - Wraps Tauri commands using `invoke()`.
// - Handles Tauri events using `listen()`.
//
// Architecture:
//
// React Components
//      ↓
// services/api.ts
//      ↓
// Tauri IPC
//      ↓
// Rust Commands
//      ↓
// Backend Services
//
// This keeps Tauri communication separate from the UI code.
// ============================================================


// ------------------------------------------------------------
// Tauri Core API
// ------------------------------------------------------------
// `invoke()` is used to call Rust functions registered with
// Tauri's `#[tauri::command]` attribute.
import { invoke } from "@tauri-apps/api/core";

// ------------------------------------------------------------
// Tauri Event API
// ------------------------------------------------------------
// `listen()` subscribes to events emitted by the Rust backend.
import { listen } from "@tauri-apps/api/event";

// ------------------------------------------------------------
// Frontend Type Definitions
// ------------------------------------------------------------
// These types describe the data exchanged between the React
// frontend and the Rust backend.
import type {
  BrowserMediaCapture,
  DownloadItem,
  MediaInfo
} from "../types";


// ============================================================
// URL ANALYSIS
// ============================================================

// Analyze a media URL using the Rust backend.
//
// Rust command:
//     analyze_url
//
// Input:
//     URL entered by the user.
//
// Output:
//     MediaInfo containing metadata and available formats.
export const analyzeUrl = (url: string) =>
  invoke<MediaInfo>("analyze_url", { url });


// ============================================================
// STANDARD DOWNLOAD
// ============================================================

// Start a normal URL-based download.
//
// Rust command:
//     start_download
//
// Parameters:
// - url: Media page or downloadable URL.
// - formatId: Optional format selected by the user.
// - outputTemplate: Optional filename/output template.
//
// `undefined` values are converted to `null` because Tauri
// commands can receive explicit null values from JavaScript.
export const startDownload = (
  url: string,
  formatId?: string,
  outputTemplate?: string
) =>
  invoke<string>("start_download", {
    url,
    formatId: formatId ?? null,
    outputTemplate: outputTemplate ?? null
  });


// ============================================================
// DOWNLOAD LIST
// ============================================================

// Retrieve all download jobs currently stored in the
// Rust application's shared state.
//
// Rust command:
//     list_downloads
//
// Returns an array of DownloadItem objects.
export const listDownloads = () =>
  invoke<DownloadItem[]>("list_downloads");


// ============================================================
// BROWSER MEDIA CAPTURES
// ============================================================

// Retrieve media detected by the Chrome extension.
//
// Rust command:
//     list_browser_captures
//
// Returns browser-captured video/audio stream information.
export const listBrowserCaptures = () =>
  invoke<BrowserMediaCapture[]>("list_browser_captures");


// ============================================================
// CAPTURED MEDIA DOWNLOAD
// ============================================================

// Start a download using browser-captured media streams.
//
// This is mainly used when the Chrome extension detects
// separate video and audio streams.
//
// Rust command:
//     start_captured_download
//
// Parameters:
// - captureId: Identifies the browser media capture.
// - videoStreamId: Selected video stream.
// - audioStreamId: Optional audio stream.
// - outputPath: Optional destination file path.
//
// The Rust backend downloads the streams and can combine
// video + audio using FFmpeg.
export const startCapturedDownload = (
  captureId: string,
  videoStreamId: string,
  audioStreamId?: string,
  outputPath?: string
) =>
  invoke<string>("start_captured_download", {
    captureId,
    videoStreamId,
    audioStreamId: audioStreamId ?? null,
    outputPath: outputPath ?? null
  });


// ============================================================
// DOWNLOAD CONTROL
// ============================================================

// Cancel an existing download job.
//
// Rust command:
//     cancel_download
export const cancelDownload = (id: string) =>
  invoke("cancel_download", { id });


// Pause an existing download job.
//
// Rust command:
//     pause_download
export const pauseDownload = (id: string) =>
  invoke("pause_download", { id });


// Resume a paused download job.
//
// Rust command:
//     resume_download
export const resumeDownload = (id: string) =>
  invoke("resume_download", { id });


// ============================================================
// BROWSER MONITORING
// ============================================================

// Enable or disable browser media monitoring.
//
// When enabled, the Chrome extension can send detected
// media information to the native host and then to the
// Tauri application.
//
// Rust command:
//     set_browser_watch
export const watchBrowser = (enabled: boolean) =>
  invoke("set_browser_watch", { enabled });


// ============================================================
// BROWSER MEDIA EVENTS
// ============================================================

// Subscribe to browser-media detection events emitted by
// the Rust backend.
//
// Rust event:
//     browser-media-detected
//
// Whenever the backend receives and processes a browser
// capture, the event payload contains a BrowserMediaCapture.
//
// The callback allows React components to immediately update
// the UI without waiting for the next polling refresh.
export const onBrowserMedia = (
  callback: (capture: BrowserMediaCapture) => void
) =>
  listen<BrowserMediaCapture>("browser-media-detected", (event) =>
    callback(event.payload)
  );