import { useEffect, useState } from "react";
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
import type { BrowserMediaCapture, DownloadItem, MediaInfo } from "./types";

function bytes(value?: number) {
  if (!value) return "Unknown";
  const units = ["B", "KB", "MB", "GB", "TB"];
  const i = Math.min(Math.floor(Math.log(value) / Math.log(1024)), units.length - 1);
  return `${(value / Math.pow(1024, i)).toFixed(i ? 1 : 0)} ${units[i]}`;
}

function duration(value?: number) {
  if (!value) return "—";
  const h = Math.floor(value / 3600);
  const m = Math.floor((value % 3600) / 60);
  const s = Math.floor(value % 60);
  return h ? `${h}:${String(m).padStart(2, "0")}:${String(s).padStart(2, "0")}` : `${m}:${String(s).padStart(2, "0")}`;
}

export default function App() {
  const [url, setUrl] = useState("");
  const [media, setMedia] = useState<MediaInfo | null>(null);
  const [downloads, setDownloads] = useState<DownloadItem[]>([]);
  const [captures, setCaptures] = useState<BrowserMediaCapture[]>([]);
  const [watching, setWatching] = useState(false);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState("");

  async function refresh() {
    try {
      setDownloads(await listDownloads());
      setCaptures(await listBrowserCaptures());
    } catch {
      // Expected when the UI is opened in a plain browser during development.
    }
  }

  useEffect(() => {
    refresh();
    const timer = window.setInterval(refresh, 1000);
    const unlisten = onBrowserMedia((capture) => {
      setCaptures((old) => [capture, ...old.filter((x) => x.captureId !== capture.captureId)]);
    });
    return () => {
      window.clearInterval(timer);
      unlisten.then((fn) => fn());
    };
  }, []);

  async function analyze() {
    if (!url.trim()) return;
    setBusy(true);
    setError("");
    try {
      setMedia(await analyzeUrl(url.trim()));
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  }

  async function download(formatId?: string) {
    setBusy(true);
    setError("");
    try {
      await startDownload(url.trim(), formatId);
      await refresh();
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  }

  async function toggleWatch() {
    try {
      const next = !watching;
      await watchBrowser(next);
      setWatching(next);
    } catch (e) {
      setError(String(e));
    }
  }

  async function captureDownload(capture: BrowserMediaCapture) {
    const videos = capture.streams.filter((x) => x.type === "video");
    const audios = capture.streams.filter((x) => x.type === "audio");
    if (!videos.length) {
      setError("No video stream is available in this browser capture.");
      return;
    }
    try {
      await startCapturedDownload(
        capture.captureId,
        videos[0].id,
        audios[0]?.id
      );
      await refresh();
    } catch (e) {
      setError(String(e));
    }
  }

  return (
    <main className="shell">
      <header className="topbar">
        <div>
          <div className="brand">Charlie MJ</div>
          <div className="subtitle">Video Downloader</div>
        </div>
        <button className={watching ? "watch active" : "watch"} onClick={toggleWatch}>
          {watching ? "● Browser monitoring" : "Watch Chrome"}
        </button>
      </header>

      <section className="hero">
        <h1>Download. Convert. Manage.</h1>
        <p>Paste a URL or let Charlie MJ capture authorized browser media and combine video + audio automatically.</p>
        <div className="url-row">
          <input
            value={url}
            onChange={(e) => setUrl(e.target.value)}
            onKeyDown={(e) => e.key === "Enter" && analyze()}
            placeholder="Paste a supported media URL..."
          />
          <button onClick={analyze} disabled={busy || !url.trim()}>
            {busy ? "Working…" : "Analyze"}
          </button>
        </div>
        {error && <div className="error">{error}</div>}
      </section>

      {media && (
        <section className="card">
          <div className="media-head">
            {media.thumbnail && <img src={media.thumbnail} alt="" />}
            <div>
              <h2>{media.title}</h2>
              <p>{media.uploader ?? "Unknown"} · {duration(media.duration)}</p>
              <div className="actions">
                <button onClick={() => download()}>Best Quality</button>
                <button className="secondary" onClick={() => download("bestaudio")}>MP3 Audio</button>
              </div>
            </div>
          </div>

          <h3>Available formats</h3>
          <div className="format-list">
            {media.formats.slice(0, 25).map((f) => (
              <button key={f.id} className="format" onClick={() => download(f.id)}>
                <span>{f.resolution ?? f.id}</span>
                <span>{f.extension}</span>
                <span>{f.fps ? `${f.fps} FPS` : ""}</span>
                <span>{bytes(f.filesize)}</span>
              </button>
            ))}
          </div>
        </section>
      )}

      {captures.length > 0 && (
        <section className="card">
          <div className="section-title">
            <h2>Browser Media Capture</h2>
            <span>{captures.length} detected</span>
          </div>

          <div className="capture-list">
            {captures.map((capture) => {
              const videos = capture.streams.filter((x) => x.type === "video");
              const audios = capture.streams.filter((x) => x.type === "audio");
              return (
                <div className="capture" key={capture.captureId}>
                  <div>
                    <strong>{capture.title || "Detected browser media"}</strong>
                    <div className="muted">{capture.source} · {videos.length} video · {audios.length} audio</div>
                    <div className="muted">{capture.pageUrl}</div>
                  </div>
                  <div className="capture-info">
                    <span>Video: {videos[0]?.quality || videos[0]?.mime || "detected"}</span>
                    <span>Audio: {audios[0]?.quality || audios[0]?.mime || "detected"}</span>
                    <button onClick={() => captureDownload(capture)}>Download & Combine</button>
                  </div>
                </div>
              );
            })}
          </div>
        </section>
      )}

      <section className="card">
        <div className="section-title">
          <h2>Downloads</h2>
          <span>{downloads.length} jobs</span>
        </div>
        <div className="download-list">
          {downloads.length === 0 && <div className="empty">No downloads yet.</div>}
          {downloads.map((item) => (
            <div className="download-item" key={item.id}>
              <div className="download-title">{item.title}</div>
              <div className="muted">
                {item.status} · {item.progress.toFixed(0)}% · {bytes(item.speedBytesPerSecond)}/s
              </div>
              <progress value={item.progress} max="100" />
              {!["completed", "cancelled", "failed"].includes(item.status) && (
                <button className="danger" onClick={() => cancelDownload(item.id)}>Cancel</button>
              )}
            </div>
          ))}
        </div>
      </section>

      <footer>Charlie MJ Video Downloader · Respect applicable service terms and copyright.</footer>
    </main>
  );
}
