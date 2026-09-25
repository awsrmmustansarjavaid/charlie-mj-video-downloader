# ============================================================
# Charlie MJ Video Downloader - Tool Setup
# ============================================================
# Purpose:
# - Create the tools/bin directory if it does not exist
# - Show where required external binaries should be placed
#
# Required tools:
# - yt-dlp.exe
# - ffmpeg.exe
# - ffprobe.exe
#
# These tools are used by the video downloader for downloading
# and processing media files.
# ============================================================


# ------------------------------------------------------------
# Script Parameters
# ------------------------------------------------------------
# Define the directory where external tool binaries will be
# stored.
#
# $PSScriptRoot = directory containing this PowerShell script.
#
# Default location:
# <project-root>\tools\bin
#
# You can provide a different directory when running the script.
# ------------------------------------------------------------
param(
    [string]$ToolsDir = "$PSScriptRoot\..\tools\bin"
)


# ------------------------------------------------------------
# Create Tools Directory
# ------------------------------------------------------------
# Create the tools directory if it does not already exist.
#
# -ItemType Directory = create a directory
# -Force              = create it if missing; do not fail if
#                       it already exists
#
# Out-Null hides the normal PowerShell output.
# ------------------------------------------------------------
New-Item -ItemType Directory -Force -Path $ToolsDir | Out-Null


# ------------------------------------------------------------
# Display Tool Location
# ------------------------------------------------------------
# Tell the user where the required executable files should
# be placed.
# ------------------------------------------------------------
Write-Host "Place pinned yt-dlp.exe, ffmpeg.exe and ffprobe.exe in:"


# ------------------------------------------------------------
# Resolve and Display Absolute Path
# ------------------------------------------------------------
# Resolve-Path converts the relative ToolsDir path into its
# full absolute filesystem path.
# ------------------------------------------------------------
Write-Host (Resolve-Path $ToolsDir)


# ------------------------------------------------------------
# Third-Party Binary Warning
# ------------------------------------------------------------
# yt-dlp and FFmpeg are third-party software.
#
# Before distributing these binaries with the application,
# review their official releases, licenses, and redistribution
# requirements.
#
# Do not blindly download or redistribute executable files
# from unknown or untrusted sources.
# ------------------------------------------------------------
Write-Host "Do not blindly download third-party binaries without reviewing their release and redistribution terms."