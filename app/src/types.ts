export type DownloadStatus =
  | "queued"
  | "analyzing"
  | "downloading"
  | "processing"
  | "paused"
  | "completed"
  | "failed"
  | "cancelled";

export interface MediaFormat {
  id: string;
  extension: string;
  resolution?: string;
  fps?: number;
  filesize?: number;
  vcodec?: string;
  acodec?: string;
  abr?: number;
  note?: string;
}

export interface MediaInfo {
  id: string;
  title: string;
  uploader?: string;
  duration?: number;
  thumbnail?: string;
  webpageUrl: string;
  formats: MediaFormat[];
  subtitles: string[];
}

export interface DownloadItem {
  id: string;
  url: string;
  title: string;
  status: DownloadStatus;
  progress: number;
  speedBytesPerSecond: number;
  totalBytes?: number;
  downloadedBytes: number;
  outputPath?: string;
  error?: string;
  createdAt: string;
}
