$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path -Parent $PSScriptRoot
$keyPath = Join-Path $repoRoot 'charlie-mj-extension.pem'

Write-Host 'Charlie MJ CRX setup verification'
Write-Host '--------------------------------'

if (-not (Get-Command node -ErrorAction SilentlyContinue)) {
    throw 'Node.js was not found. The project already requires Node.js for its normal app build.'
}
Write-Host "Node.js: $((node --version).Trim())"

if (-not (Test-Path $keyPath)) {
    Write-Host 'CRX key: MISSING'
    Write-Host 'Run: powershell -ExecutionPolicy Bypass -File .\scripts\generate-crx-key.ps1'
    exit 1
}

$firstLine = (Get-Content $keyPath -TotalCount 1).Trim()
if ($firstLine -notmatch '^-----BEGIN (RSA )?PRIVATE KEY-----$') {
    throw "CRX key is invalid or is not PEM format: $keyPath"
}

$id = node (Join-Path $PSScriptRoot 'extension-id.js') $keyPath
if ($id.Length -ne 32 -or $id -notmatch '^[a-p]{32}$') {
    throw "Unable to calculate a valid Chrome extension ID: $id"
}

Write-Host "CRX key: OK"
Write-Host "Chrome extension ID: $id"
Write-Host 'OpenSSL: NOT REQUIRED'
Write-Host 'Do not commit the PEM file.'
