// ============================================================
// Charlie MJ Video Downloader - Main React Application
// ============================================================
// Purpose:
// - Provides the main user interface for the downloader.
// - Allows users to analyze media URLs.
// - Displays available video/audio formats.
// - Starts normal downloads.
// - Receives browser-captured media from the Chrome extension.
// - Allows captured video + audio streams to be combined.
// - Displays download jobs and their current status.
//
// Frontend flow:
//
// React UI
//   ↓
// services/api.ts
//   ↓
// Tauri Commands
//   ↓
// Rust Backend
//
// Browser capture flow:
//
// Chrome Extension
//   ↓
// Native Messaging Host
//   ↓
// Rust IPC Server
//   ↓
// Browser Media Event
//   ↓
// React App
// ============================================================


// React hooks used for component state and lifecycle management.
import { useEffect, useState } from "react";

// Application API functions used to communicate with the
// Tauri/Rust backend.
import {
  analyzeUrl,
  cancelDownload,
  listBrowserCaptures,
  listDownloads,
  onBrowserMedia,
  startCapturedDownload,
  startDownload,
  watchBrowser
} from "./services/api";

// TypeScript data models used by the frontend.
import type {
  BrowserMediaCapture,
  DownloadItem,
  MediaInfo
} from "./types";


// ============================================================
// Format File Size
// ============================================================
// Converts a byte value into a human-readable size.
//
// Examples:
// 1024       → 1 KB
// 1048576    → 1 MB
// 1073741824 → 1 GB
//
// If no value is available, "Unknown" is returned.
// ============================================================
function bytes(value?: number) {
  if (!value) return "Unknown";

  // Supported file-size units.
  const units = ["B", "KB", "MB", "GB", "TB"];

  // Calculate which unit should be displayed.
  const i = Math.min(
    Math.floor(Math.log(value) / Math.log(1024)),
    units.length - 1
  );

  // Convert the value to the selected unit.
  return `${(value / Math.pow(1024, i)).toFixed(i ? 1 : 0)} ${units[i]}`;
}


// ============================================================
// Format Duration
// ============================================================
// Converts seconds into a readable media duration.
//
// Examples:
// 65 seconds    → 1:05
// 3665 seconds  → 1:01:05
//
// If no duration is available, "—" is returned.
// ============================================================
function duration(value?: number) {
  if (!value) return "—";

  // Calculate hours, minutes, and seconds.
  const h = Math.floor(value / 3600);
  const m = Math.floor((value % 3600) / 60);
  const s = Math.floor(value % 60);

  // Include hours only when the duration is at least one hour.
  return h
    ? `${h}:${String(m).padStart(2, "0")}:${String(s).padStart(2, "0")}`
    : `${m}:${String(s).padStart(2, "0")}`;
}


// ============================================================
// Main Application Component
// ============================================================
// App is the main React component for Charlie MJ Video
// Downloader.
//
// It contains:
// - URL analysis
// - Media information
// - Available formats
// - Browser media capture
// - Download management
// - Browser monitoring control
// ============================================================
export default function App() {

  // ----------------------------------------------------------
  // URL Input
  // ----------------------------------------------------------
  // Stores the URL entered by the user.
  const [url, setUrl] = useState("");


  // ----------------------------------------------------------
  // Analyzed Media
  // ----------------------------------------------------------
  // Stores metadata returned by the Rust backend after
  // analyzing the entered media URL.
  const [media, setMedia] = useState<MediaInfo | null>(null);


  // ----------------------------------------------------------
  // Download Jobs
  // ----------------------------------------------------------
  // Stores the list of downloads currently known by the app.
  const [downloads, setDownloads] = useState<DownloadItem[]>([]);


  // ----------------------------------------------------------
  // Browser Captures
  // ----------------------------------------------------------
  // Stores media detected by the Chrome extension.
  //
  // Each capture can contain one or more video/audio streams.
  const [captures, setCaptures] = useState<BrowserMediaCapture[]>([]);


  // ----------------------------------------------------------
  // Browser Monitoring State
  // ----------------------------------------------------------
  // true  = browser monitoring is enabled
  // false = browser monitoring is disabled
  const [watching, setWatching] = useState(false);


  // ----------------------------------------------------------
  // Busy State
  // ----------------------------------------------------------
  // Used to prevent duplicate operations while an action such
  // as media analysis or download initialization is running.
  const [busy, setBusy] = useState(false);


  // ----------------------------------------------------------
  // Error Message
  // ----------------------------------------------------------
  // Stores an error message that can be displayed to the user.
  const [error, setError] = useState("");


  // ==========================================================
  // Refresh Application Data
  // ==========================================================
  // Retrieves the latest download jobs and browser captures
  // from the Rust backend.
  //
  // This function is also called periodically by useEffect().
  // ==========================================================
  async function refresh() {
    try {

      // Retrieve current download jobs.
      setDownloads(await listDownloads());

      // Retrieve browser media captures.
      setCaptures(await listBrowserCaptures());

    } catch {

      // This can happen when the React UI is opened directly
      // in a normal browser instead of inside the Tauri app.
      //
      // The error is intentionally ignored during development.
    }
  }


  // ==========================================================
  // Application Initialization
  // ==========================================================
  // Runs once when the App component is mounted.
  //
  // Responsibilities:
  // - Load initial application data.
  // - Refresh downloads/captures every second.
  // - Listen for new browser-media events.
  // - Clean up timers and event listeners when unmounted.
  // ==========================================================
  useEffect(() => {

    // Load initial download and browser-capture data.
    refresh();


    // Refresh application state every second.
    const timer = window.setInterval(refresh, 1000);


    // Listen for browser media captured by the Chrome extension.
    const unlisten = onBrowserMedia((capture) => {

      // Add the newest capture to the beginning of the list.
      //
      // If the same capture already exists, remove the old copy
      // before adding the updated version.
      setCaptures((old) => [
        capture,
        ...old.filter((x) => x.captureId !== capture.captureId)
      ]);
    });


    // Cleanup when the component is removed.
    return () => {

      // Stop the periodic refresh timer.
      window.clearInterval(timer);

      // Remove the browser-media event listener.
      unlisten.then((fn) => fn());
    };

  }, []);


  // ==========================================================
  // Analyze Media URL
  // ==========================================================
  // Sends the entered URL to the Rust backend for analysis.
  //
  // The backend can return:
  // - Title
  // - Uploader
  // - Duration
  // - Thumbnail
  // - Available formats
  // - Subtitles
  // ==========================================================
  async function analyze() {

    // Do nothing when the URL field is empty.
    if (!url.trim()) return;

    // Show busy state while analysis is running.
    setBusy(true);

    // Clear any previous error.
    setError("");

    try {

      // Analyze the trimmed URL using the backend API.
      setMedia(await analyzeUrl(url.trim()));

    } catch (e) {

      // Display the backend error to the user.
      setError(String(e));

    } finally {

      // Always clear the busy state after the operation.
      setBusy(false);
    }
  }


  // ==========================================================
  // Start Normal Download
  // ==========================================================
  // Starts a download using the analyzed media URL.
  //
  // If formatId is provided, that specific format is requested.
  // If no formatId is provided, the backend uses its default
  // best-quality selection.
  // ==========================================================
  // Download a selected format. If the selected format is video-only,
  // automatically pair it with the best available audio stream so the
  // user receives one combined media file instead of separate files.
  function downloadFormat(format: { id: string; acodec?: string | null }) {
    const videoOnly = !format.acodec || format.acodec === "none";
    return download(videoOnly ? `${format.id}+bestaudio` : format.id);
  }


  async function download(formatId?: string) {

    // Show busy state.
    setBusy(true);

    // Clear previous errors.
    setError("");

    try {

      // Start the download through the Tauri backend.
      await startDownload(url.trim(), formatId);

      // Refresh the download list after creating the job.
      await refresh();

    } catch (e) {

      // Display any download-start error.
      setError(String(e));

    } finally {

      // Clear busy state.
      setBusy(false);
    }
  }


  // ==========================================================
  // Toggle Browser Monitoring
  // ==========================================================
  // Enables or disables browser media monitoring.
  //
  // The actual monitoring is handled by the browser extension
  // and backend. This function changes the monitoring state
  // through the application API.
  // ==========================================================
  async function toggleWatch() {
    try {

      // Calculate the new monitoring state.
      const next = !watching;

      // Tell the backend to enable/disable browser monitoring.
      await watchBrowser(next);

      // Update the UI state.
      setWatching(next);

    } catch (e) {

      // Display any error returned by the backend.
      setError(String(e));
    }
  }


  // ==========================================================
  // Download Captured Browser Media
  // ==========================================================
  // Starts a download using streams captured from the browser.
  //
  // A capture can contain:
  // - Video streams
  // - Audio streams
  //
  // The first available video and audio streams are currently
  // selected and passed to the backend.
  // ==========================================================
  async function captureDownload(capture: BrowserMediaCapture) {

    // Find all video streams in the capture.
    const videos = capture.streams.filter(
      (x) => x.type === "video"
    );

    // Find all audio streams in the capture.
    const audios = capture.streams.filter(
      (x) => x.type === "audio"
    );


    // A video stream is required for this download method.
    if (!videos.length) {
      setError(
        "No video stream is available in this browser capture."
      );
      return;
    }


    try {

      // Start the captured-media download.
      //
      // The first video stream is selected.
      // The first audio stream is optional.
      //
      // The Rust backend downloads the streams separately and
      // uses FFmpeg to combine them into the final media file.
      await startCapturedDownload(
        capture.captureId,
        videos[0].id,
        audios[0]?.id
      );


      // Refresh the download list.
      await refresh();

    } catch (e) {

      // Display any backend error.
      setError(String(e));
    }
  }


  // ==========================================================
  // Render Application UI
  // ==========================================================
  return (
    <main className="shell">

      {/* ------------------------------------------------------
          Application Header
          ------------------------------------------------------
          Displays the application branding and browser
          monitoring control.
      ------------------------------------------------------ */}
      <header className="topbar">

        <div>
          <div className="brand">Charlie MJ</div>
          <div className="subtitle">Video Downloader</div>
        </div>


        {/* Browser monitoring toggle button. */}
        <button
          className={watching ? "watch active" : "watch"}
          onClick={toggleWatch}
        >
          {watching
            ? "● Browser monitoring"
            : "Watch Chrome"}
        </button>

      </header>


      {/* ------------------------------------------------------
          Hero / URL Analysis Section
          ------------------------------------------------------ */}
      <section className="hero">

        <h1>Download. Convert. Manage.</h1>

        <p>
          Paste a URL or let Charlie MJ capture authorized browser
          media and combine video + audio automatically.
        </p>


        {/* URL input and Analyze button. */}
        <div className="url-row">

          <input
            value={url}

            // Update URL state whenever the user types.
            onChange={(e) => setUrl(e.target.value)}

            // Pressing Enter starts URL analysis.
            onKeyDown={(e) =>
              e.key === "Enter" && analyze()
            }

            placeholder="Paste a supported media URL..."
          />


          <button
            onClick={analyze}
            disabled={busy || !url.trim()}
          >
            {busy ? "Working…" : "Analyze"}
          </button>

        </div>


        {/* Display an error when one exists. */}
        {error && <div className="error">{error}</div>}

      </section>


      {/* ======================================================
          Analyzed Media Section
          ======================================================
          Displayed only after media analysis succeeds.
      ====================================================== */}
      {media && (
        <section className="card">

          <div className="media-head">

            {/* Display the media thumbnail when available. */}
            {media.thumbnail && (
              <img src={media.thumbnail} alt="" />
            )}


            <div>

              {/* Media title. */}
              <h2>{media.title}</h2>


              {/* Uploader and media duration. */}
              <p>
                {media.uploader ?? "Unknown"} ·{" "}
                {duration(media.duration)}
              </p>


              {/* Quick download actions. */}
              <div className="actions">

                {/* Download using the default/best-quality selection. */}
                <button onClick={() => download()}>
                  Best Quality
                </button>


                {/* Request audio-only download. */}
                <button
                  className="secondary"
                  onClick={() => download("bestaudio")}
                >
                  MP3 Audio
                </button>

              </div>

            </div>

          </div>


          {/* Available media formats returned by the backend. */}
          <h3>Available formats</h3>


          <div className="format-list">

            {/* Display at most the first 25 formats. */}
            {media.formats.slice(0, 25).map((f) => (

              <button
                key={f.id}
                className="format"

                // Video-only formats are automatically paired with the
                // best available audio stream and merged by FFmpeg.
                onClick={() => downloadFormat(f)}
              >

                {/* Resolution or format ID. */}
                <span>
                  {f.resolution ?? f.id}
                </span>


                {/* File extension. */}
                <span>{f.extension}</span>


                {/* Frame rate when available. */}
                <span>
                  {f.fps ? `${f.fps} FPS` : ""}
                </span>


                {/* Estimated/known file size. */}
                <span>{bytes(f.filesize)}</span>

              </button>

            ))}

          </div>

        </section>
      )}


      {/* ======================================================
          Browser Media Capture Section
          ======================================================
          Displayed when the Chrome extension has detected
          browser media.
      ====================================================== */}
      {captures.length > 0 && (
        <section className="card">

          <div className="section-title">

            <h2>Browser Media Capture</h2>

            {/* Number of detected captures. */}
            <span>
              {captures.length} detected
            </span>

          </div>


          <div className="capture-list">

            {/* Render every captured browser-media item. */}
            {captures.map((capture) => {

              // Extract video streams from this capture.
              const videos = capture.streams.filter(
                (x) => x.type === "video"
              );


              // Extract audio streams from this capture.
              const audios = capture.streams.filter(
                (x) => x.type === "audio"
              );


              return (
                <div
                  className="capture"
                  key={capture.captureId}
                >

                  <div>

                    {/* Capture title. */}
                    <strong>
                      {capture.title ||
                        "Detected browser media"}
                    </strong>


                    {/* Capture source and stream counts. */}
                    <div className="muted">
                      {capture.source} ·{" "}
                      {videos.length} video ·{" "}
                      {audios.length} audio
                    </div>


                    {/* Original browser page URL. */}
                    <div className="muted">
                      {capture.pageUrl}
                    </div>

                  </div>


                  <div className="capture-info">

                    {/* Show information about the first video stream. */}
                    <span>
                      Video:{" "}
                      {videos[0]?.quality ||
                        videos[0]?.mime ||
                        "detected"}
                    </span>


                    {/* Show information about the first audio stream. */}
                    <span>
                      Audio:{" "}
                      {audios[0]?.quality ||
                        audios[0]?.mime ||
                        "detected"}
                    </span>


                    {/* Start video + audio download and combine operation. */}
                    <button
                      onClick={() => captureDownload(capture)}
                    >
                      Download & Combine
                    </button>

                  </div>

                </div>
              );

            })}

          </div>

        </section>
      )}


      {/* ======================================================
          Download Management Section
          ====================================================== */}
      <section className="card">

        <div className="section-title">

          <h2>Downloads</h2>

          {/* Number of download jobs currently stored. */}
          <span>
            {downloads.length} jobs
          </span>

        </div>


        <div className="download-list">

          {/* Display a message when there are no downloads. */}
          {downloads.length === 0 && (
            <div className="empty">
              No downloads yet.
            </div>
          )}


          {/* Render each download job. */}
          {downloads.map((item) => (

            <div
              className="download-item"
              key={item.id}
            >

              {/* Download title. */}
              <div className="download-title">
                {item.title}
              </div>


              {/* Download status, progress, and speed. */}
              <div className="muted">

                {item.status} ·{" "}

                {item.progress.toFixed(0)}% ·{" "}

                {bytes(item.speedBytesPerSecond)}/s

              </div>


              {/* Visual download progress bar. */}
              <progress
                value={item.progress}
                max="100"
              />

              {item.error && (
                <div className="error">{item.error}</div>
              )}

              {item.outputPath && item.status === "completed" && (
                <div className="muted">Saved: {item.outputPath}</div>
              )}

              {/* Show Cancel only while the job is not finished. */}
              {![
                "completed",
                "cancelled",
                "failed"
              ].includes(item.status) && (

                <button
                  className="danger"
                  onClick={() =>
                    cancelDownload(item.id)
                  }
                >
                  Cancel
                </button>

              )}

            </div>

          ))}

        </div>

      </section>


      {/* ------------------------------------------------------
          Application Footer
          ------------------------------------------------------ */}
      <footer>
        Charlie MJ Video Downloader · Respect applicable service
        terms and copyright.
      </footer>

    </main>
  );
}