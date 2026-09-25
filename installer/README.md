# Windows Installer

The Tauri NSIS bundle produces the primary setup executable.

Target name:

```text
Charlie-MJ-Video-Downloader-Setup.exe
```

The release package should contain:

- Desktop application
- `tools/yt-dlp.exe`
- `tools/ffmpeg.exe`
- `tools/ffprobe.exe`
- Native host executable
- Native host manifest
- Browser integration onboarding files

Browser extensions should be installed through supported browser mechanisms. Do not silently alter browser configuration on consumer systems.
