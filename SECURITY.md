# Security Policy

Please report security vulnerabilities privately to the repository maintainers rather than opening a public issue.

Security-sensitive areas include:

- Native Messaging
- localhost IPC
- URL handling
- subprocess invocation
- cookie/authentication storage
- bundled binaries
- installer behavior

Never execute arbitrary shell commands derived from browser messages or downloaded metadata.
