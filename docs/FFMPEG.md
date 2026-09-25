# FFmpeg Pipeline

Preferred flow:

```text
video.tmp + audio.tmp
       ↓
codec/container inspection
       ↓
compatible?
 ┌─────┴─────┐
 yes         no
 ↓           ↓
-c copy    controlled re-encode
 └─────┬─────┘
       ↓
   final.mp4
       ↓
   ffprobe
```

Stream copy avoids unnecessary quality loss and is normally much faster.

For incompatible streams, the fallback in this repository uses H.264/AAC as a broadly compatible MP4 target. Production releases should make this configurable and expose clear progress/status.
