# Charlie MJ Video Downloader

> **Download. Convert. Manage.**

Charlie MJ Video Downloader is a Windows desktop download manager with two complementary media paths:

1. **Direct/Web Download** — analyze supported URLs through yt-dlp and download the selected format.
2. **Browser Media Capture** — detect authorized media streams from Chrome/Edge, send stream metadata to the desktop application, download video/audio separately when necessary, and automatically mux them into one final file with FFmpeg.

## Final feature set

### Video & audio
- Video downloads from sites supported by the configured extraction engine
- 144p → 8K when the source exposes those formats
- Best Quality / Best Video / Best Audio profiles
- MP4/WebM/MKV and source formats where supported
- MP3, M4A, WAV, FLAC and OPUS audio extraction
- Playlist and batch URL architecture
- Subtitle, thumbnail, chapter and metadata options
- FPS/codec/format information
- Smart output-container selection
- Stream-copy muxing when possible
- Controlled re-encode fallback
- ffprobe output verification

### Browser capture
- Chrome/Chromium Manifest V3
- Edge-compatible extension architecture
- "Download with Charlie MJ" context menu
- Browser page URL capture
- Google Drive `videoplayback` detection
- Video/audio stream detection
- Stream parameter normalization
- Stream deduplication
- Automatic video/audio pairing
- "Download & Combine"
- Watch Browser mode
- Native Messaging bridge
- Lightweight extension; desktop performs the heavy download/merge work

### Download manager
- Queue
- Priority
- Pause/resume state
- Cancellation
- Retry architecture
- Concurrent jobs
- Progress reporting
- Speed reporting
- Scheduler foundation
- Clipboard workflow foundation
- History/SQLite foundation
- Automatic temporary-file cleanup
- Download verification

### Windows
- Tauri 2
- Rust backend
- React + TypeScript UI
- SQLite persistence
- FFmpeg + ffprobe
- yt-dlp
- NSIS installer
- GitHub Actions CI/release
- Portable-friendly directory layout

## Architecture

```text
                  ┌──────────────────────┐
                  │ Chrome / Edge         │
                  │ Manifest V3          │
                  └──────────┬───────────┘
                             │
                    Native Messaging
                             │
                             ▼
                  ┌──────────────────────┐
                  │ Native Host          │
                  └──────────┬───────────┘
                             │ localhost
                             ▼
┌──────────────┐     ┌──────────────────────┐
│ Paste URL    │────►│ Charlie MJ Desktop   │
└──────────────┘     │ Tauri + React + Rust │
                     └──────────┬───────────┘
                                │
                  ┌─────────────┼─────────────┐
                  ▼             ▼             ▼
               yt-dlp       Downloader      Browser
                  │             │            Capture
                  │             │             │
                  └─────────────┼─────────────┘
                                ▼
                         Temp Video/Audio
                                │
                                ▼
                            FFmpeg
                                │
                                ▼
                         ffprobe verify
                                │
                                ▼
                          Final MP4/MKV
```

## Important compatibility and legal note

The project supports media that the configured engines and browser session are authorized to access. Website implementations, authentication, regional restrictions and technical protections can affect compatibility. Do not use this project to bypass DRM, access controls, paywalls, or other technical protections, and respect applicable laws and service terms.

## Development

Install Rust, Node.js LTS and the Tauri Windows prerequisites.

Place development media tools here:

```text
tools/bin/yt-dlp.exe
tools/bin/ffmpeg.exe
tools/bin/ffprobe.exe
```

Then:

```powershell
cd app
npm install
npm run tauri dev
```

Build:

```powershell
npm run tauri build
```

## Browser development

1. Build/run the desktop application.
2. Load `browser-extension/` as an unpacked extension.
3. Build/install the native host.
4. Replace the placeholder extension ID in the native-host manifest.
5. Open an authorized media page.
6. The extension sends detected media to Charlie MJ.

## Production release checklist

- Pin tested yt-dlp and FFmpeg versions.
- Review all third-party licenses and redistribution terms.
- Sign the installer and binaries.
- Restrict Native Messaging origins to the official extension ID.
- Add IPC authentication in production.
- Test clean Windows 10/11 VMs.
- Publish SHA-256 checksums.
- Publish third-party notices.
- Do not silently install browser extensions outside supported browser/enterprise mechanisms.

## License

The application source in this repository is MIT licensed. Third-party tools and dependencies retain their own licenses. See `THIRD-PARTY-NOTICES.md`.


## Release artifacts

The Windows release workflow (`.github/workflows/release.yml`) produces:

- `Charlie-MJ-Video-Downloader-Setup-x64.exe`
- `Charlie-MJ-Video-Downloader-Chrome-Extension.crx`
- `Charlie-MJ-Video-Downloader-Chrome-Extension.zip`
- `SHA256SUMS.txt`

The workflow downloads pinned-at-build-time runtime tools into the Tauri resource bundle:

- `yt-dlp.exe`
- `ffmpeg.exe`
- `ffprobe.exe`

The desktop application resolves these from Tauri's packaged resource directory instead of assuming that `yt-dlp` is installed globally.

### Stable CRX signing

Create one RSA private key for the Chrome extension and keep it outside Git. Base64-encode the PEM and add it to GitHub Actions as:

`CHARLIE_MJ_CRX_PRIVATE_KEY_B64`

The release workflow derives the stable extension ID from that key and places it into the Native Messaging configuration.

Do not commit the private key.

### Native Messaging

The Windows application registers the Charlie MJ Native Messaging host for the current Windows user at first launch. It does not modify Chrome policies or attempt to bypass browser security controls.

### Browser permissions

The extension uses Manifest V3 and requests broad host access because media capture is performed from user-authorized pages. The extension should be published/reviewed through the Chrome Web Store for normal distribution. The CRX is also provided as a release artifact for controlled/developer installations.

### Runtime and legal notes

The application only downloads media that the user is authorized to access and should be used in accordance with the relevant site's terms and copyright law. It does not implement DRM circumvention or security-control bypasses.


## Runtime packaging notes

- The Windows installer bundles `yt-dlp.exe`, `ffmpeg.exe`, and `ffprobe.exe` under the Tauri application resources directory. The desktop app resolves these bundled tools from the packaged resource directory, so end users do not need to install them separately.
- The CRX release is a CRX3 package signed with RSA/SHA-256 using Chromium-compatible PKCS#1 v1.5 signing. The packer self-verifies the signature before creating the release artifact.
