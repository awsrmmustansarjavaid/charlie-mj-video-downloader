# Scripts

Add project automation here as the release process grows.

Recommended scripts:

- download/pin yt-dlp
- download/pin FFmpeg
- build native host
- register native host
- package extension
- generate/verify CRX signing key without OpenSSL
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
## Chrome CRX signing key (no OpenSSL)

This repository does **not** require OpenSSL for Chrome extension signing.

Run from PowerShell at the repository root:

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\generate-crx-key.ps1
```

The key is created as:

```text
charlie-mj-extension.pem
```

The file is ignored by Git and must remain private. Keep a secure backup if you want the Chrome extension ID to remain stable across machines/releases.

To package the extension locally:

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\package-extension.ps1
```

Requirements for local CRX packaging are **Node.js and Google Chrome**, both of which are already used by this repository's development/build workflow. OpenSSL is not required.

If Chrome is unavailable, the extension ZIP can still be loaded through Chrome's developer-mode **Load unpacked** workflow.

## GitHub Actions signing key

The release workflow no longer fails when `CHARLIE_MJ_CRX_PRIVATE_KEY_B64` is missing. It automatically generates a temporary RSA signing key with the Node.js crypto implementation already used by the repository, so **OpenSSL is never required and no manual command is required to make the GitHub release run**.

If you want the Chrome extension ID to stay identical across every release, configure the repository secret `CHARLIE_MJ_CRX_PRIVATE_KEY_B64` with the same private key every time. Without that secret, the workflow intentionally uses a new temporary key and therefore the generated CRX has a new extension identity.

Never commit a private signing key or a base64-encoded private key to the repository.
