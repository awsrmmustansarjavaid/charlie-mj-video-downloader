# Release setup

## Required GitHub Actions secret

`CHARLIE_MJ_CRX_PRIVATE_KEY_B64`

Generate a stable RSA PEM key once, base64 encode the complete PEM, and store the base64 value as the secret. Never commit the private key.

Example PowerShell:

```powershell
openssl genrsa -out charlie-mj-extension.pem 2048
[Convert]::ToBase64String([IO.File]::ReadAllBytes("charlie-mj-extension.pem"))
```

Copy the resulting base64 string into the GitHub Actions secret.

## Optional Tauri signing secrets

For signed Tauri updater/release artifacts, configure the normal Tauri signing secrets:

- `TAURI_SIGNING_PRIVATE_KEY`
- `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`

Windows Authenticode signing should be configured separately with an appropriate certificate. Signing reduces SmartScreen friction but cannot guarantee that every security product will trust a new application immediately.

## Release

Push a version tag such as:

```text
v0.2.1
```

The workflow builds the Windows installer, packages the MV3 extension as CRX and ZIP, calculates SHA-256 checksums, and publishes all artifacts to the GitHub Release.
