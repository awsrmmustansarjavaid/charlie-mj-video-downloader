# Release Checklist

- [ ] Pin yt-dlp version
- [ ] Pin FFmpeg version
- [ ] Test Windows 10
- [ ] Test Windows 11
- [ ] Test clean install
- [ ] Test uninstall
- [ ] Test upgrade
- [ ] Test browser extension registration
- [ ] Test Native Messaging
- [ ] Test direct URL
- [ ] Test browser-captured video
- [ ] Test video-only
- [ ] Test audio-only
- [ ] Test video + audio mux
- [ ] Test failed download cleanup
- [ ] Test ffprobe verification
- [ ] Generate SHA-256
- [ ] Sign installer
- [ ] Publish third-party notices


## Runtime packaging notes

- The Windows installer bundles `yt-dlp.exe`, `ffmpeg.exe`, and `ffprobe.exe` under the Tauri application resources directory. The desktop app resolves these bundled tools from the packaged resource directory, so end users do not need to install them separately.
- The CRX release is a CRX3 package signed with RSA/SHA-256 using Chromium-compatible PKCS#1 v1.5 signing. The packer self-verifies the signature before creating the release artifact.

### v5.1 build fix
- Fixed Rust `E0382` compile error by cloning the download job ID before moving it into the background Tokio task.
- The command now returns the original job ID while the background task owns its clone.
