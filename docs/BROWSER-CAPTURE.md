# Browser Capture

## Detection

The extension watches completed HTTP(S) requests and classifies:

- `video/*`
- `audio/*`
- Google Drive-style `videoplayback`
- common direct video/audio file extensions

For Drive-style URLs it reads useful query metadata when present and removes temporary request parameters:

```text
range
rn
rbuf
ump
srfvp
```

## Pairing

The desktop application receives multiple streams. A future production selector should score candidates using:

- stream type
- resolution
- FPS
- bitrate
- MIME/container
- duration
- tab/page identity
- source
- size

Then show the user:

```text
1080p + 128kbps
720p  + 128kbps
480p  + 96kbps
```

## Session lifetime

Captured browser stream URLs may expire. They should be consumed quickly rather than treated as permanent download URLs.
