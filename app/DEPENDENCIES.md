# Dependency Installation

The repository intentionally does not ship a generated npm lockfile in this initial source package.

Run:

```powershell
cd app
npm install
```

Then commit the generated `package-lock.json` after reviewing dependency versions for the project release.

---
# Dependency flow

```
React / TypeScript
        │
        ▼
      Tauri
        │
        ▼
   Rust Backend
        │
        ├── Tokio ────────── Async tasks
        ├── Reqwest ──────── HTTP/downloads
        ├── Futures Util ─── Streaming
        ├── URL ──────────── URL processing
        │
        ├── Rusqlite ─────── SQLite database
        ├── Serde/JSON ───── Data exchange
        ├── UUID ─────────── Unique IDs
        ├── Chrono ───────── Dates/timestamps
        ├── SHA2 + Hex ───── File/hash verification
        │
        └── Tauri Plugins
              ├── Dialog
              ├── Filesystem
              ├── Notifications
              └── System Tray
```

This gives your Cargo.toml a clear documentation layer while keeping the actual dependency configuration unchanged.
---
## browser_capture.rs

### File responsibility

```
Chrome Extension
       │
       │ media_detected JSON
       ▼
Native Messaging Host
       │
       ▼
browser_capture.rs
       │
       ├── Validate message type
       ├── Read source/page/title
       ├── Validate stream URL
       ├── Normalize Google video URL
       ├── Detect video/audio type
       ├── Extract quality/size/duration
       ├── Remove duplicate streams
       ├── Generate UUIDs
       └── Create BrowserMediaCapture
              │
              ▼
        Rust/Tauri Backend
```

One useful detail: browser_capture.rs is essentially the bridge between the browser's raw JSON media-detection data and your application's strongly structured Rust models.
---
## What commands.rs does in your project

The file is essentially the command/API layer between your React frontend and Rust backend:

```
React / TypeScript UI
        │
        │ Tauri Commands
        ▼
   commands.rs
        │
        ├── analyze_url()
        │       ↓
        │    media.rs
        │
        ├── start_download()
        │       ↓
        │    media.rs
        │
        ├── start_captured_download()
        │       ↓
        │    downloader.rs
        │       ↓
        │    ffmpeg.rs
        │
        ├── list_downloads()
        ├── list_browser_captures()
        ├── pause_download()
        ├── resume_download()
        ├── cancel_download()
        └── set_browser_watch()
```

One important implementation note: pause_download(), resume_download(), and cancel_download() currently change the job's status in memory; they do not actually pause/resume/terminate the running downloader process. The comments above intentionally describe the current behavior rather than claiming those controls are fully implemented.
---
## database.rs

### Database structure

Your current database is organized into three main tables:

```
SQLite Database
│
├── downloads
│   ├── id
│   ├── url
│   ├── title
│   ├── status
│   ├── progress
│   ├── speed
│   ├── total_bytes
│   ├── downloaded_bytes
│   ├── output_path
│   └── error
│
├── browser_captures
│   ├── capture_id
│   ├── source
│   ├── page_url
│   ├── title
│   ├── payload_json
│   └── created_at
│
└── settings
    ├── key
    └── value
```

One technical point: because this uses a Rust raw string (r#"..."#), the // lines inside the SQL block are passed to SQLite as SQL text. SQLite does support -- comments, but not Rust-style // comments. So if you compile this exact version, those comments inside execute_batch() should be changed to SQL comments (--) or kept outside the SQL string.
---
## downloader.rs role in your application

```
Chrome Extension
       │
       ▼
Native Messaging Host
       │
       ▼
Browser Capture
       │
       ▼
commands.rs
       │
       ▼
downloader.rs
       │
       ├── Download Video → video.tmp
       │
       └── Download Audio → audio.tmp
                    │
                    ▼
                 ffmpeg.rs
                    │
                    ▼
              Final .mp4 file
```

One small existing-code detail: Instant::now() is currently assigned to _started but never used, so it doesn't currently calculate download speed or duration. Your existing code is still valid; this would only become useful when you add real-time progress/speed reporting.
---
## ffmpeg.rs role in your application

```
                downloader.rs
                     │
             ┌───────┴───────┐
             ▼               ▼
         video.tmp       audio.tmp
             │               │
             └───────┬───────┘
                     ▼
                  ffmpeg.rs
                     │
             ┌───────┴────────┐
             │                │
          -c copy          Fallback
             │             libx264
             │               +
             │              AAC
             └───────┬────────┘
                     ▼
                 final.mp4
                     │
                     ▼
                  ffprobe
                     │
                     ▼
              Verified Media
```

One useful detail: your mux() has a good two-stage approach: it first tries -c copy, which avoids re-encoding, and only re-encodes if that fails. This is particularly useful for your browser-captured separate video/audio streams because compatible streams can be combined quickly without quality loss.
---
## ipc.rs

### How this file fits into your project

```
Chrome Extension
      │
      │ Native Messaging
      ▼
native-host/host.py
      │
      │ HTTP POST
      ▼
ipc.rs
127.0.0.1:47821
      │
      ▼
browser_capture.rs
      │
      ├── Validate message
      ├── Extract video/audio streams
      └── Create BrowserMediaCapture
      │
      ▼
AppState
      │
      ▼
Tauri Event
"browser-media-detected"
      │
      ▼
React Frontend
      │
      ▼
User selects video/audio
      │
      ▼
start_captured_download()
      │
      ▼
downloader.rs → ffmpeg.rs
```

One important implementation note: this IPC server currently performs very lightweight HTTP parsing—it assumes the complete request arrives in a single socket.read() and takes everything after \r\n\r\n as the body. That is fine for a small lab/prototype, but for a production version you would want proper Content-Length handling, request-size limits, and possibly a small HTTP library.
---
## lib.rs

lib.rs role in your project

Think of lib.rs as the main backend wiring file:

```

                    lib.rs
                      │
       ┌──────────────┼──────────────┐
       │              │              │
    AppState       Commands        Plugins
       │              │
       │              ├── analyze_url
       │              ├── start_download
       │              ├── captured_download
       │              ├── pause/resume
       │              └── browser_watch
       │
       └──────────────┐
                      │
                Tauri Backend
                      │
       ┌──────────────┼───────────────┐
       │              │               │
    media.rs     downloader.rs    ffmpeg.rs
       │              │               │
       └──────────────┴───────────────┘
                      │
                  Final Media
```

And your browser-capture path is:

```
Chrome Extension
       ↓
host.py
       ↓
HTTP POST
       ↓
ipc.rs
       ↓
browser_capture.rs
       ↓
AppState.captures
       ↓
Tauri Event
       ↓
React UI
```

One thing to keep in mind: lib.rs currently starts the IPC server automatically during application setup, so opening Charlie MJ starts the local browser-to-desktop communication endpoint as well.
---
## main.rs

### Simple architecture

```
main.rs
   │
   │ calls
   ▼
lib.rs
   │
   ├── AppState
   ├── Tauri Builder
   ├── Plugins
   ├── Commands
   ├── IPC Server
   │
   ▼
Tauri Application
   │
   ├── React Frontend
   └── Rust Backend
```

So, main.rs is intentionally very small. Its main job is simply to start charlie_mj_video_downloader_lib::run().
---
## media.rs

### media.rs in your architecture

```
                 React Frontend
                       │
                       ▼
                  commands.rs
                       │
             ┌─────────┴─────────┐
             ▼                   ▼
       analyze_url()       start_download()
             │                   │
             └─────────┬─────────┘
                       ▼
                    media.rs
                       │
                       ▼
                  yt-dlp.exe
                       │
          ┌────────────┴────────────┐
          ▼                         ▼
     analyze()                  download()
          │                         │
          ▼                         ▼
     MediaInfo                 Media File
     ├── title                 ├── MP4
     ├── formats               ├── MP3
     ├── subtitles             └── Selected format
     └── thumbnail
```

### One important distinction

Your project now has two different download paths:

```
URL Download
    ↓
commands.rs
    ↓
media.rs
    ↓
yt-dlp.exe

and browser-captured media:

Chrome Extension
    ↓
host.py
    ↓
ipc.rs
    ↓
browser_capture.rs
    ↓
commands.rs
    ↓
downloader.rs
    ↓
ffmpeg.rs
    ↓
Final MP4
```

So media.rs is primarily your yt-dlp-based URL downloader/analyzer, while downloader.rs + ffmpeg.rs handle the browser-captured stream download and merging path.
---
## models.rs

### models.rs role

This file is essentially the data contract between your Rust backend and frontend.

```
                    models.rs
                       │
       ┌───────────────┼────────────────┐
       │               │                │
       ▼               ▼                ▼
 MediaInfo       BrowserMediaCapture  DownloadItem
       │               │                │
       ▼               ▼                ▼
MediaFormat     CapturedStream       Download UI
       │               │
       ▼               ▼
   yt-dlp         Chrome Extension
```

The #[serde(rename = "...")] attributes are particularly important because your Rust naming convention uses snake_case, while your JavaScript/TypeScript side uses camelCase. For example:

```
Rust                    JSON / Frontend
────────────────────────────────────────
webpage_url       →     webpageUrl
tab_id            →     tabId
page_url          →     pageUrl
capture_id        →     captureId
created_at        →     createdAt
output_path       →     outputPath
```

This lets you keep idiomatic Rust code without requiring changes to your frontend JSON naming.

---
## state.rs

### How AppState fits into your application

```
                    Tauri Application
                           │
                           ▼
                      ┌─────────┐
                      │ AppState│
                      └────┬────┘
                           │
          ┌────────────────┼────────────────┐
          │                │                │
          ▼                ▼                ▼
     Downloads        Browser Captures   Browser Watch
     DownloadItem     BrowserMedia       true / false
                      Capture
                           │
                           ▼
                      Data Directory
```

In your lib.rs, this state is created and registered with Tauri:

```
let state = AppState {
    downloads: Mutex::new(Vec::new()),
    captures: Mutex::new(Vec::new()),
    browser_watch: Mutex::new(false),
    data_dir: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
};

tauri::Builder::default()
    .manage(state)
```

So the overall flow is:

lib.rs → creates AppState → Tauri manages it → commands/IPC access shared state.

---
## App.tsx

### App.tsx role in your architecture

```
                    App.tsx
                       │
          ┌────────────┼────────────┐
          │            │            │
          ▼            ▼            ▼
      URL Analyze   Downloads   Browser Capture
          │            │            │
          └────────────┼────────────┘
                       ▼
                  services/api.ts
                       │
                       ▼
                 Tauri Commands
                       │
                       ▼
                  Rust Backend
```

The important point is that App.tsx is primarily your UI/orchestration layer. It doesn't perform the actual downloading itself. It calls services/api.ts, which communicates with your Rust/Tauri backend where the actual analysis, downloading, browser capture handling, and FFmpeg processing happen.

---
##  main.tsx

main.tsx role in your project

```
index.html
    │
    │ <div id="root">
    ▼
main.tsx
    │
    ├── Loads styles.css
    │
    ├── Creates React root
    │
    └── Loads App.tsx
             │
             ▼
      Charlie MJ UI
             │
             ▼
       services/api.ts
             │
             ▼
       Tauri / Rust Backend
```

So, main.tsx is the frontend entry point. It doesn't contain your downloader logic; its main job is to start React and load App.tsx.

---
## styles.css

### styles.css role

Your frontend structure is now essentially:

```
index.html
    │
    ▼
main.tsx
    │
    ▼
App.tsx
    │
    ├── UI structure
    │
    └── CSS classes
            │
            ▼
       styles.css
            │
            ├── Theme
            ├── Buttons
            ├── Header
            ├── URL analyzer
            ├── Media cards
            ├── Format list
            ├── Browser captures
            ├── Download list
            ├── Progress bars
            └── Footer
```

So styles.css is the presentation layer of your React frontend. It controls how the UI looks, while App.tsx controls what the UI does.

---
## types.ts

### types.ts role in your architecture

```
                    Rust Backend
                         │
                         │ JSON / Tauri data
                         ▼
                  services/api.ts
                         │
                         ▼
                     types.ts
                         │
          ┌──────────────┼──────────────┐
          ▼              ▼              ▼
      MediaInfo     BrowserMedia     DownloadItem
          │            Capture            │
          │              │                │
          └──────────────┼────────────────┘
                         ▼
                      App.tsx
                         │
                         ▼
                    React UI
```

The important role of this file is type safety and data consistency. For example, DownloadStatus prevents the UI from accidentally using an unsupported status, while MediaInfo, CapturedStream, and DownloadItem define the exact data shape expected by App.tsx.

---
## api.ts

### api.ts role in your project

```
React UI
   │
   │ calls functions
   ▼
services/api.ts
   │
   │ Tauri invoke()
   ▼
Rust/Tauri Commands
   │
   ├── analyze_url
   ├── start_download
   ├── start_captured_download
   ├── list_downloads
   ├── list_browser_captures
   ├── cancel_download
   ├── pause_download
   ├── resume_download
   └── set_browser_watch
   │
   ▼
Rust Backend
```

### For browser detection, the direction is reversed:

```
Chrome Extension
      ↓
Native Messaging Host
      ↓
ipc.rs
      ↓
browser_capture.rs
      ↓
Tauri Event
      ↓
"browser-media-detected"
      ↓
api.ts
      ↓
App.tsx
      ↓
Browser Media Capture UI
```

So, api.ts is the bridge between your React frontend and the Rust/Tauri backend. It keeps all Tauri invoke() and event-listening code in one place instead of putting backend communication directly inside App.tsx.

---
