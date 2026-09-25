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


## Release package

The GitHub release workflow produces both:

- `Charlie-MJ-Video-Downloader-Chrome-Extension.crx`
- `Charlie-MJ-Video-Downloader-Chrome-Extension.zip`

The extension is Manifest V3. It adds a small floating Charlie MJ control to normal web pages and reports detected authorized media to the desktop application through Chrome Native Messaging.

The desktop application performs the actual download and FFmpeg merge. The extension does not attempt DRM circumvention or security-control bypasses.

For normal consumer distribution, publish the extension through the Chrome Web Store. The CRX is useful for controlled/developer installations and release testing.
