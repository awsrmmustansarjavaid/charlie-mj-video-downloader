# Charlie MJ release verification (PowerShell)

$ErrorActionPreference = "Stop"

$required = @(
  "browser-extension\manifest.json",
  "browser-extension\background.js",
  "browser-extension\content.js",
  "browser-extension\overlay.css",
  "native-host\host.py",
  "app\src-tauri\tauri.conf.json",
  ".github\workflows\release.yml"
)

foreach ($path in $required) {
  if (-not (Test-Path $path)) {
    throw "Missing required file: $path"
  }
}

$manifest = Get-Content "browser-extension\manifest.json" -Raw | ConvertFrom-Json
if ($manifest.manifest_version -ne 3) { throw "Extension must use Manifest V3." }

$tauri = Get-Content "app\src-tauri\tauri.conf.json" -Raw | ConvertFrom-Json
if (-not $tauri.bundle.resources) { throw "Tauri runtime resources are not configured." }

Write-Host "Charlie MJ source verification passed."
