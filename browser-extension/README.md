# Charlie MJ Browser Extension

Manifest V3 Chromium extension.

## What it does

- Watches completed browser media requests.
- Detects `video/*` and `audio/*` responses.
- Detects Google Drive-style `videoplayback` requests.
- Reads `mime`, `clen`, `dur`, `itag`/quality information where present.
- Removes temporary range/request parameters for Google Drive-style media URLs.
- Deduplicates streams.
- Sends stream metadata to the Charlie MJ Native Messaging host.
- Provides a context-menu "Download with Charlie MJ" action.

The extension is intentionally lightweight. It does not download the full media into the extension.

## Development

1. Run the desktop app.
2. Open `chrome://extensions`.
3. Enable Developer Mode.
4. Load this directory as an unpacked extension.
5. Install/configure the native messaging host.
6. Replace the placeholder extension ID in the native-host manifest.

For production, request only the browser permissions required by the final implementation and follow Chrome/Edge extension policies.
