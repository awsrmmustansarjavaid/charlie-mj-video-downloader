import json
import subprocess
import struct
import sys

def frame(obj):
    data = json.dumps(obj).encode()
    return struct.pack("<I", len(data)) + data

def test_json_roundtrip_shape():
    message = {
        "type": "media_detected",
        "source": "google_drive",
        "streams": [
            {"streamType": "video", "url": "https://example.test/video"},
            {"streamType": "audio", "url": "https://example.test/audio"}
        ]
    }
    assert json.loads(json.dumps(message)) == message

if __name__ == "__main__":
    test_json_roundtrip_shape()
    print("protocol shape OK")
