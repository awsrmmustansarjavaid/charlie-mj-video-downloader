import json
import os
import struct
import sys
import urllib.request

IPC_URL = os.environ.get(
    "CHARLIE_MJ_IPC_URL",
    "http://127.0.0.1:47821/"
)

def read_message():
    length_bytes = sys.stdin.buffer.read(4)
    if len(length_bytes) != 4:
        return None
    length = struct.unpack("<I", length_bytes)[0]
    payload = sys.stdin.buffer.read(length)
    if len(payload) != length:
        return None
    return json.loads(payload.decode("utf-8"))

def write_message(message):
    payload = json.dumps(message).encode("utf-8")
    sys.stdout.buffer.write(struct.pack("<I", len(payload)))
    sys.stdout.buffer.write(payload)
    sys.stdout.buffer.flush()

def valid_url(value):
    return isinstance(value, str) and (
        value.startswith("http://") or value.startswith("https://")
    )

def post_to_desktop(message):
    if message.get("type") == "open_url":
        if not valid_url(message.get("url")):
            return {"ok": False, "error": "Invalid URL"}
    elif message.get("type") == "media_detected":
        for stream in message.get("streams", []):
            if not valid_url(stream.get("url")):
                return {"ok": False, "error": "Invalid stream URL"}
    else:
        return {"ok": False, "error": "Unsupported message type"}

    body = json.dumps(message).encode("utf-8")
    request = urllib.request.Request(
        IPC_URL,
        data=body,
        headers={"Content-Type": "application/json"},
        method="POST"
    )

    with urllib.request.urlopen(request, timeout=3) as response:
        return {"ok": response.status < 300}

def main():
    while True:
        message = read_message()
        if message is None:
            break

        try:
            write_message(post_to_desktop(message))
        except Exception as exc:
            write_message({"ok": False, "error": str(exc)})

if __name__ == "__main__":
    main()
