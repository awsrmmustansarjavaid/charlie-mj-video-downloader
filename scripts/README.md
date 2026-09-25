# Scripts

Add project automation here as the release process grows.

Recommended scripts:

- download/pin yt-dlp
- download/pin FFmpeg
- build native host
- register native host
- package extension
- generate checksums
- generate third-party notices
- verify release bundle

---
# What this script does

```
Run PowerShell script
        │
        ▼
Find project directory
        │
        ▼
Create tools/bin/
        │
        ▼
Show absolute tools location
        │
        ▼
User places:
  ├── yt-dlp.exe
  ├── ffmpeg.exe
  └── ffprobe.exe
```

## Your resulting project structure can be:

```
Charlie-MJ-Video-Downloader/
│
├── app/
│   └── src-tauri/
│
├── tools/
│   └── bin/
│       ├── yt-dlp.exe
│       ├── ffmpeg.exe
│       ├── ffprobe.exe
│       └── .gitkeep
│
└── scripts/
    └── setup-tools.ps1
```

### The .gitignore you provided earlier will also prevent those .exe files from accidentally being committed:

```
tools/bin/*.exe
!tools/bin/.gitkeep
```