import json
import os
import struct
import sys
import urllib.request

IPC_URL = os.environ.get("CHARLIE_MJ_IPC_URL", "http://127.0.0.1:47821/open")

def read_message():
    raw_length = sys.stdin.buffer.read(4)
    if len(raw_length) != 4:
        return None
    length = struct.unpack("<I", raw_length)[0]
    data = sys.stdin.buffer.read(length)
    return json.loads(data.decode("utf-8"))

def write_message(message):
    data = json.dumps(message).encode("utf-8")
    sys.stdout.buffer.write(struct.pack("<I", len(data)))
    sys.stdout.buffer.write(data)
    sys.stdout.buffer.flush()

def main():
    while True:
        message = read_message()
        if message is None:
            break

        if message.get("type") != "open_url":
            write_message({"ok": False, "error": "Unsupported message type"})
            continue

        url = message.get("url", "")
        if not (url.startswith("http://") or url.startswith("https://")):
            write_message({"ok": False, "error": "Invalid URL"})
            continue

        payload = json.dumps({"url": url}).encode("utf-8")
        try:
            request = urllib.request.Request(
                IPC_URL,
                data=payload,
                headers={"Content-Type": "application/json"},
                method="POST",
            )
            with urllib.request.urlopen(request, timeout=3) as response:
                write_message({"ok": response.status < 300})
        except Exception as exc:
            write_message({"ok": False, "error": str(exc)})

if __name__ == "__main__":
    main()
