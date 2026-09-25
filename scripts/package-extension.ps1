$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path -Parent $PSScriptRoot
$extensionDir = Join-Path $repoRoot 'browser-extension'
$keyPath = Join-Path $repoRoot 'charlie-mj-extension.pem'
$releaseDir = Join-Path $repoRoot 'release'

if (-not (Test-Path $extensionDir)) { throw "Missing browser-extension directory: $extensionDir" }

if (-not (Test-Path $keyPath)) {
    & (Join-Path $PSScriptRoot 'generate-crx-key.ps1')
}

$chromeCandidates = @(
    (Join-Path $env:ProgramFiles 'Google\Chrome\Application\chrome.exe'),
    (Join-Path ${env:ProgramFiles(x86)} 'Google\Chrome\Application\chrome.exe'),
    (Join-Path $env:LOCALAPPDATA 'Google\Chrome\Application\chrome.exe')
)
$chrome = $chromeCandidates | Where-Object { $_ -and (Test-Path $_) } | Select-Object -First 1
if (-not $chrome) {
    throw "Google Chrome was not found. Install/use Chrome to create a CRX package, or use the extension ZIP for manual loading. No OpenSSL is required."
}

New-Item -ItemType Directory -Force -Path $releaseDir | Out-Null
$stage = Join-Path $env:TEMP 'charlie-mj-extension-package'
if (Test-Path $stage) { Remove-Item $stage -Recurse -Force }
New-Item -ItemType Directory -Force -Path $stage | Out-Null
Copy-Item (Join-Path $extensionDir '*') $stage -Recurse -Force

& $chrome "--pack-extension=$stage" "--pack-extension-key=$keyPath"
$crx = "$stage.crx"
if (-not (Test-Path $crx)) { throw "Chrome did not generate a CRX file: $crx" }

Copy-Item $crx (Join-Path $releaseDir 'Charlie-MJ-Video-Downloader-Chrome-Extension.crx') -Force
Compress-Archive -Path (Join-Path $extensionDir '*') -DestinationPath (Join-Path $releaseDir 'Charlie-MJ-Video-Downloader-Chrome-Extension.zip') -Force
Write-Host "CRX and extension ZIP created in: $releaseDir"
