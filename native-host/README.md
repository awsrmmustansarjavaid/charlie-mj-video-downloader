# Native Messaging Host

This bridge receives JSON from the Chromium extension and forwards approved URLs to the local Charlie MJ application.

## Production requirements

- Build the Python host into a Windows executable.
- Install the host manifest through the Windows installer.
- Replace the placeholder extension ID.
- Bind the desktop IPC server to loopback only.
- Add an authentication token to the IPC protocol.
- Validate all incoming URLs.
- Never execute arbitrary commands received from the browser.
