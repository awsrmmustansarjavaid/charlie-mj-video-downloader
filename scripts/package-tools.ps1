param(
  [string]$ToolsDir = "$PSScriptRoot\..\tools\bin"
)

New-Item -ItemType Directory -Force -Path $ToolsDir | Out-Null

Write-Host "Place pinned yt-dlp.exe, ffmpeg.exe and ffprobe.exe in:"
Write-Host (Resolve-Path $ToolsDir)
Write-Host "Do not blindly download third-party binaries without reviewing their release and redistribution terms."
