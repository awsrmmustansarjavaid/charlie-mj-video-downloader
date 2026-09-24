# Architecture

```text
Chrome / Edge
      |
      v
Manifest V3 Extension
      |
      v
Native Messaging Host
      |
      v
Local Charlie MJ Desktop IPC
      |
      v
Tauri / Rust
 |       |        |
 v       v        v
SQLite  yt-dlp   FFmpeg
 |
 v
Queue / History / Settings
 |
 v
Windows filesystem
```

## Responsibility boundaries

### Browser extension

- Capture page/link/media URL.
- Provide context menu.
- Send URL to native host.
- Never store passwords.
- Never execute arbitrary local commands.

### Native host

- Read browser messages.
- Validate message shape.
- Forward URL to desktop loopback endpoint.

### Rust desktop backend

- Validate URLs.
- Manage queue and application state.
- Invoke yt-dlp.
- Coordinate FFmpeg.
- Persist state in SQLite.
- Enforce concurrency and cancellation.
- Expose safe Tauri commands.

### React frontend

- UI only.
- URL analyzer.
- Format picker.
- Queue/history/settings views.
