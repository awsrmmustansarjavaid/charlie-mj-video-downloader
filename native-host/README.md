# Native Messaging Host

The host accepts browser JSON messages and forwards them to Charlie MJ over localhost.

Supported messages:

```json
{
  "type": "open_url",
  "url": "https://example.test/video"
}
```

and:

```json
{
  "type": "media_detected",
  "source": "google_drive",
  "pageUrl": "https://drive.google.com/...",
  "title": "Example video",
  "streams": [
    {
      "streamType": "video",
      "url": "https://...",
      "mime": "video/mp4",
      "quality": "1080p",
      "size": 123456
    },
    {
      "streamType": "audio",
      "url": "https://...",
      "mime": "audio/mp4",
      "quality": "128kbps",
      "size": 12345
    }
  ]
}
```

Production requirements:

- Build with PyInstaller or an equivalent trusted packaging method.
- Install the manifest in the browser's Native Messaging location.
- Replace the extension ID placeholder.
- Add authenticated IPC before public release.
- Never execute browser-provided commands.

---

## Build flow

```
host.py
   │
   ▼
PyInstaller
   │
   │ --onefile
   │ --name host
   ▼
native-host/dist/host.exe
```

---
## Communication architecture

```
┌──────────────────────────────┐
│      Chrome Extension        │
│                              │
│ Detect media / URL            │
└──────────────┬───────────────┘
               │
               │ Chrome Native Messaging
               │ JSON + 4-byte length
               ▼
┌──────────────────────────────┐
│          host.py             │
│                              │
│ • Read message               │
│ • Validate URL               │
│ • Validate message type      │
│ • Forward request            │
└──────────────┬───────────────┘
               │
               │ HTTP POST
               │ 127.0.0.1:47821
               ▼
┌──────────────────────────────┐
│ Charlie MJ Desktop App       │
│                              │
│ Tauri / Rust                 │
│                              │
│ • Receive IPC request        │
│ • Process download           │
│ • Manage media               │
└──────────────────────────────┘
```

This keeps your Chrome extension → Python native host → Tauri/Rust desktop app architecture clear and makes the code much easier to maintain.

---

