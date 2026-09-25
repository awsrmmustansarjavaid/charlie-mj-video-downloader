import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { BrowserMediaCapture, DownloadItem, MediaInfo } from "../types";

export const analyzeUrl = (url: string) =>
  invoke<MediaInfo>("analyze_url", { url });

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

export const listDownloads = () =>
  invoke<DownloadItem[]>("list_downloads");

export const listBrowserCaptures = () =>
  invoke<BrowserMediaCapture[]>("list_browser_captures");

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

export const cancelDownload = (id: string) =>
  invoke("cancel_download", { id });

export const pauseDownload = (id: string) =>
  invoke("pause_download", { id });

export const resumeDownload = (id: string) =>
  invoke("resume_download", { id });

export const watchBrowser = (enabled: boolean) =>
  invoke("set_browser_watch", { enabled });

export const onBrowserMedia = (
  callback: (capture: BrowserMediaCapture) => void
) =>
  listen<BrowserMediaCapture>("browser-media-detected", (event) =>
    callback(event.payload)
  );
