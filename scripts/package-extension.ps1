$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path -Parent $PSScriptRoot
$extensionDir = Join-Path $repoRoot 'browser-extension'
$keyPath = Join-Path $repoRoot 'charlie-mj-extension.pem'
$releaseDir = Join-Path $repoRoot 'release'
$zipPath = Join-Path $env:TEMP 'Charlie-MJ-Video-Downloader-Chrome-Extension.zip'
$crxPath = Join-Path $env:TEMP 'Charlie-MJ-Video-Downloader-Chrome-Extension.crx'

if (-not (Test-Path $extensionDir)) { throw "Missing browser-extension directory: $extensionDir" }
if (-not (Get-Command node -ErrorAction SilentlyContinue)) { throw 'Node.js is required. OpenSSL and Google Chrome are not required.' }

if (-not (Test-Path $keyPath)) {
    & (Join-Path $PSScriptRoot 'generate-crx-key.ps1')
}

New-Item -ItemType Directory -Force -Path $releaseDir | Out-Null
Remove-Item $zipPath, $crxPath -Force -ErrorAction SilentlyContinue

Compress-Archive -Path (Join-Path $extensionDir '*') -DestinationPath $zipPath -Force
& node (Join-Path $PSScriptRoot 'package-crx3.js') $keyPath $zipPath $crxPath

if (-not (Test-Path $crxPath)) { throw 'CRX3 package was not created.' }
Copy-Item $crxPath (Join-Path $releaseDir 'Charlie-MJ-Video-Downloader-Chrome-Extension.crx') -Force
Copy-Item $zipPath (Join-Path $releaseDir 'Charlie-MJ-Video-Downloader-Chrome-Extension.zip') -Force
Write-Host 'CRX3 and extension ZIP created without Chrome or OpenSSL.'
