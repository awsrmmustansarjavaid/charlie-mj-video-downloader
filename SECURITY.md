# Security Policy

Security-sensitive components include:

- Browser Native Messaging
- Loopback IPC
- URL parsing
- yt-dlp/FFmpeg subprocess invocation
- Cookie/authentication data
- Installer registration
- Downloaded metadata

Rules:

- Treat browser input as untrusted.
- Allow only HTTP/HTTPS media URLs.
- Never execute a command supplied by a browser message.
- Keep IPC on 127.0.0.1.
- Add an authentication token before production release.
- Restrict native-host origins to the official extension ID.
- Never store browser credentials in plaintext.
