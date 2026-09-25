$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path -Parent $PSScriptRoot
$keyPath = Join-Path $repoRoot 'charlie-mj-extension.pem'

if (Test-Path $keyPath) {
    Write-Host "CRX signing key already exists: $keyPath"
    exit 0
}

if (-not (Get-Command node -ErrorAction SilentlyContinue)) {
    throw "Node.js is required to generate the CRX key. This repository does not require OpenSSL."
}

node (Join-Path $PSScriptRoot 'generate-crx-key.js') $keyPath
if (-not (Test-Path $keyPath)) {
    throw "CRX signing key was not created."
}

Write-Host "CRX signing key created successfully. Do NOT commit the .pem file."
