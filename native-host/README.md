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
