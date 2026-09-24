import { invoke } from "@tauri-apps/api/core";
import type { DownloadItem, MediaInfo } from "../types";

export async function analyzeUrl(url: string): Promise<MediaInfo> {
  return invoke<MediaInfo>("analyze_url", { url });
}

export async function startDownload(
  url: string,
  formatId?: string,
  outputTemplate?: string
): Promise<string> {
  return invoke<string>("start_download", {
    url,
    formatId: formatId ?? null,
    outputTemplate: outputTemplate ?? null
  });
}

export async function listDownloads(): Promise<DownloadItem[]> {
  return invoke<DownloadItem[]>("list_downloads");
}

export async function cancelDownload(id: string): Promise<void> {
  return invoke("cancel_download", { id });
}

export async function pauseDownload(id: string): Promise<void> {
  return invoke("pause_download", { id });
}

export async function resumeDownload(id: string): Promise<void> {
  return invoke("resume_download", { id });
}
