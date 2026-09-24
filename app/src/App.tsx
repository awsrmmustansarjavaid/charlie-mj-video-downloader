import { useEffect, useMemo, useState } from "react";
import type { DownloadItem, MediaInfo } from "./types";
import { analyzeUrl, cancelDownload, listDownloads, startDownload } from "./services/api";

function formatBytes(bytes: number) {
  if (!bytes) return "0 B";
  const units = ["B", "KB", "MB", "GB", "TB"];
  const index = Math.min(Math.floor(Math.log(bytes) / Math.log(1024)), units.length - 1);
  return `${(bytes / Math.pow(1024, index)).toFixed(index === 0 ? 0 : 1)} ${units[index]}`;
}

function formatDuration(seconds?: number) {
  if (!seconds) return "—";
  const h = Math.floor(seconds / 3600);
  const m = Math.floor((seconds % 3600) / 60);
  const s = Math.floor(seconds % 60);
  return h ? `${h}:${String(m).padStart(2, "0")}:${String(s).padStart(2, "0")}` : `${m}:${String(s).padStart(2, "0")}`;
}

export default function App() {
  const [url, setUrl] = useState("");
  const [media, setMedia] = useState<MediaInfo | null>(null);
  const [downloads, setDownloads] = useState<DownloadItem[]>([]);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState("");

  async function refresh() {
    try {
      setDownloads(await listDownloads());
    } catch {
      // Backend may not be available during pure browser development.
    }
  }

  useEffect(() => {
    refresh();
    const timer = window.setInterval(refresh, 1000);
    return () => window.clearInterval(timer);
  }, []);

  const videoFormats = useMemo(
    () => media?.formats.filter((f) => f.vcodec && f.vcodec !== "none") ?? [],
    [media]
  );

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
    if (!url.trim()) return;
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

  async function cancel(id: string) {
    await cancelDownload(id);
    await refresh();
  }

  return (
    <main className="app-shell">
      <header className="topbar">
        <div>
          <div className="brand">Charlie MJ</div>
          <div className="subtitle">Video Downloader</div>
        </div>
        <div className="status-pill">Windows Desktop</div>
      </header>

      <section className="hero">
        <h1>Download. Convert. Manage.</h1>
        <p>Analyze a media URL, choose the format, and send it to your download queue.</p>
        <div className="url-row">
          <input
            value={url}
            onChange={(e) => setUrl(e.target.value)}
            onKeyDown={(e) => e.key === "Enter" && analyze()}
            placeholder="Paste a supported video or media URL..."
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
              <p>{media.uploader ?? "Unknown uploader"} · {formatDuration(media.duration)}</p>
              <div className="actions">
                <button onClick={() => download()}>Best Quality</button>
                <button className="secondary" onClick={() => download("bestaudio")}>Audio</button>
              </div>
            </div>
          </div>

          <h3>Available video formats</h3>
          <div className="format-list">
            {videoFormats.slice(0, 20).map((format) => (
              <button key={format.id} className="format" onClick={() => download(format.id)}>
                <span>{format.resolution ?? format.id}</span>
                <span>{format.extension}</span>
                <span>{format.fps ? `${format.fps} FPS` : ""}</span>
                <span>{format.filesize ? formatBytes(format.filesize) : "Size unknown"}</span>
              </button>
            ))}
          </div>
        </section>
      )}

      <section className="card">
        <div className="section-title">
          <h2>Downloads</h2>
          <span>{downloads.length} items</span>
        </div>
        {downloads.length === 0 ? (
          <div className="empty">Your download queue is empty.</div>
        ) : (
          <div className="download-list">
            {downloads.map((item) => (
              <div className="download-item" key={item.id}>
                <div className="download-title">{item.title}</div>
                <div className="download-meta">
                  {item.status} · {item.progress.toFixed(0)}% · {formatBytes(item.speedBytesPerSecond)}/s
                </div>
                <progress value={item.progress} max="100" />
                {!["completed", "cancelled", "failed"].includes(item.status) && (
                  <button className="danger" onClick={() => cancel(item.id)}>Cancel</button>
                )}
              </div>
            ))}
          </div>
        )}
      </section>

      <footer>
        Charlie MJ Video Downloader · Built for Windows · Use responsibly and respect site terms and copyright.
      </footer>
    </main>
  );
}
