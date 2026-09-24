# Charlie MJ Browser Extension

Manifest V3 Chromium extension for Chrome/Edge-compatible development.

## Development

1. Open `chrome://extensions`.
2. Enable Developer mode.
3. Load `browser-extension/` as an unpacked extension.
4. Install the Charlie MJ Native Messaging host separately.

The extension does not perform media extraction itself. It sends a URL to the desktop application.

For production, replace the placeholder native host configuration with your signed, officially published extension ID and package the host during the Windows installer process.
