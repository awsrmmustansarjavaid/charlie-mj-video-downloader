# Architecture

## Two media paths

### Direct URL

```text
URL
 ↓
yt-dlp
 ↓
Format selection
 ↓
Download
 ↓
FFmpeg
 ↓
ffprobe
 ↓
Final file
```

### Browser capture

```text
Chrome / Edge
 ↓
Manifest V3 webRequest
 ↓
Stream parser
 ↓
Native Messaging
 ↓
localhost IPC
 ↓
Rust Browser Capture Engine
 ↓
Video + Audio pairing
 ↓
Direct HTTP download
 ↓
FFmpeg Smart Merge
 ↓
ffprobe verification
 ↓
Final file
```

## Browser Capture Engine

The engine is deliberately separated into:

- `BrowserMediaDetector` — extension-side request observation.
- `MediaStreamParser` — converts request metadata into normalized stream objects.
- `MediaStreamSelector` — selects a video/audio pair.
- `StreamDownloader` — downloads temporary streams.
- `FFmpegMuxer` — combines them.
- `MediaVerifier` — validates the result.

## Google Drive

Google Drive-style media can expose temporary `videoplayback` requests. The extension detects video/audio MIME values and metadata such as `clen`, `dur` and `itag` when present, then normalizes known temporary request parameters and sends the resulting stream metadata to the desktop application.

The application must still operate only on media the user's current browser session is authorized to access.

## Smart Merge

1. Download video temporary file.
2. Download audio temporary file if selected.
3. Try FFmpeg stream copy.
4. If the pair is incompatible, use controlled conversion.
5. Verify the output using ffprobe.
6. Delete temporary files only after successful processing.
