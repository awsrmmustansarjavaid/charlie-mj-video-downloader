# Changelog

## 0.2.0

- Integrated browser media capture architecture
- Added Google Drive videoplayback stream detection
- Added generic browser stream normalization
- Added automatic video/audio pairing
- Added browser-to-desktop media event flow
- Added direct stream download engine
- Added FFmpeg smart mux pipeline
- Added ffprobe verification
- Added temporary-file cleanup
- Added Watch Browser UI
- Expanded queue and job state model
- Added Windows release documentation

## 0.1.0

- Initial Tauri + React + Rust foundation

- Fixed packaged Windows runtime tool discovery for yt-dlp/FFmpeg resources.
- Fixed CRX3 RSA/SHA-256 signing compatibility and added self-verification before release.

## 0.2.2 - Permanent runtime/download fix

- Fixed the Chrome extension's broken placeholder PNG icon by shipping valid 16/48/128 PNG assets.
- Added explicit extension toolbar icons for all supported sizes.
- Fixed normal URL downloads being stuck forever at `queued / 0%` by tracking background yt-dlp success/failure.
- Normal URL downloads now default to the user's Windows Downloads folder.
- Bundled FFmpeg is explicitly passed to yt-dlp for packaged Windows builds.
- The UI now displays download errors and the final saved file path.
- Browser-captured downloads now use the same tracked job ID and update to completed/failed.
- Preserved exact Google video playback URLs instead of stripping signed query parameters.

### v5.1 build fix
- Fixed Rust `E0382` compile error by cloning the download job ID before moving it into the background Tokio task.
- The command now returns the original job ID while the background task owns its clone.
