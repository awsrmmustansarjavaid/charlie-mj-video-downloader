# ============================================================
# Charlie MJ Video Downloader - Native Messaging Host
# ============================================================
# Purpose:
# - Communicate between the Chrome extension and the desktop app
# - Receive messages from the browser extension through stdin
# - Validate URLs and media stream information
# - Forward valid messages to the Charlie MJ desktop application
#   through a local HTTP IPC endpoint
#
# Communication flow:
#
# Chrome Extension
#       │
#       │ Native Messaging
#       ▼
#    host.py
#       │
#       │ HTTP POST
#       ▼
# Charlie MJ Desktop App
#
# The native messaging protocol uses a 4-byte message length
# followed by a UTF-8 encoded JSON message.
# ============================================================


# ------------------------------------------------------------
# Import Required Python Modules
# ------------------------------------------------------------

# json:
# Used to encode and decode JSON messages.
import json

# os:
# Used to read environment variables.
import os

# struct:
# Used to encode/decode the 4-byte message length required by
# the Chrome Native Messaging protocol.
import struct

# sys:
# Used to read messages from stdin and send responses through
# stdout.
import sys

# urllib.request:
# Used to send HTTP POST requests to the local desktop app.
import urllib.request


# ------------------------------------------------------------
# Desktop Application IPC URL
# ------------------------------------------------------------
# The native host forwards messages to the Charlie MJ desktop
# application through a local HTTP endpoint.
#
# The environment variable allows the IPC address to be
# customized without changing the source code.
#
# Default:
# http://127.0.0.1:47821/
#
# 127.0.0.1 means the local computer only.
# ------------------------------------------------------------
IPC_URL = os.environ.get(
    "CHARLIE_MJ_IPC_URL",
    "http://127.0.0.1:47821/"
)


# ============================================================
# Read Message
# ============================================================
def read_message():
    """
    Read one Chrome Native Messaging message from stdin.

    Native Messaging message format:

        [4-byte message length][JSON payload]

    The first 4 bytes contain the size of the JSON payload.
    The payload is then decoded as UTF-8 and converted into
    a Python object using json.loads().
    """

    # Read the first 4 bytes containing the message length.
    length_bytes = sys.stdin.buffer.read(4)

    # If fewer than 4 bytes are received, the input stream has
    # ended or the message is incomplete.
    if len(length_bytes) != 4:
        return None

    # Decode the message length as an unsigned 32-bit integer.
    #
    # "<I":
    #   < = little-endian byte order
    #   I = unsigned 32-bit integer
    length = struct.unpack("<I", length_bytes)[0]

    # Read the JSON payload using the length received above.
    payload = sys.stdin.buffer.read(length)

    # Make sure the complete payload was received.
    if len(payload) != length:
        return None

    # Decode UTF-8 JSON and return the resulting Python object.
    return json.loads(payload.decode("utf-8"))


# ============================================================
# Write Message
# ============================================================
def write_message(message):
    """
    Send a response back to the Chrome extension.

    The response uses the Chrome Native Messaging format:

        [4-byte message length][JSON payload]
    """

    # Convert the Python object into JSON and then encode it
    # as UTF-8 bytes.
    payload = json.dumps(message).encode("utf-8")

    # Write the payload length as a 4-byte unsigned integer.
    sys.stdout.buffer.write(struct.pack("<I", len(payload)))

    # Write the actual JSON payload.
    sys.stdout.buffer.write(payload)

    # Immediately send the response instead of waiting for the
    # output buffer to fill.
    sys.stdout.buffer.flush()


# ============================================================
# Validate URL
# ============================================================
def valid_url(value):
    """
    Check whether a value is a basic HTTP/HTTPS URL.

    Only HTTP and HTTPS URLs are accepted.

    Returns:
        True  -> valid HTTP/HTTPS URL
        False -> invalid value or unsupported protocol
    """

    return isinstance(value, str) and (
        value.startswith("http://") or value.startswith("https://")
    )


# ============================================================
# Forward Message to Desktop Application
# ============================================================
def post_to_desktop(message):
    """
    Validate an incoming extension message and forward it to
    the Charlie MJ desktop application.

    Supported message types:

    1. open_url
       - Opens/handles a requested URL.

    2. media_detected
       - Contains one or more detected media streams.

    Any other message type is rejected.
    """

    # --------------------------------------------------------
    # Handle open_url messages
    # --------------------------------------------------------
    if message.get("type") == "open_url":

        # Make sure the requested URL is a valid HTTP/HTTPS URL.
        if not valid_url(message.get("url")):
            return {
                "ok": False,
                "error": "Invalid URL"
            }


    # --------------------------------------------------------
    # Handle media_detected messages
    # --------------------------------------------------------
    elif message.get("type") == "media_detected":

        # Check every detected media stream.
        for stream in message.get("streams", []):

            # Every stream must contain a valid HTTP/HTTPS URL.
            if not valid_url(stream.get("url")):
                return {
                    "ok": False,
                    "error": "Invalid stream URL"
                }


    # --------------------------------------------------------
    # Reject unsupported message types
    # --------------------------------------------------------
    else:
        return {
            "ok": False,
            "error": "Unsupported message type"
        }


    # --------------------------------------------------------
    # Convert the validated message into JSON bytes
    # --------------------------------------------------------
    body = json.dumps(message).encode("utf-8")


    # --------------------------------------------------------
    # Create HTTP POST Request
    # --------------------------------------------------------
    # The message is forwarded to the local Charlie MJ desktop
    # application's IPC server.
    # --------------------------------------------------------
    request = urllib.request.Request(
        IPC_URL,
        data=body,
        headers={
            "Content-Type": "application/json"
        },
        method="POST"
    )


    # --------------------------------------------------------
    # Send Request to Desktop Application
    # --------------------------------------------------------
    # A 3-second timeout prevents the native host from waiting
    # indefinitely if the desktop application is unavailable.
    # --------------------------------------------------------
    with urllib.request.urlopen(request, timeout=3) as response:

        # Treat HTTP status codes below 300 as successful.
        return {
            "ok": response.status < 300
        }


# ============================================================
# Main Message Loop
# ============================================================
def main():
    """
    Continuously receive messages from the Chrome extension,
    process them, and send responses back.

    The loop ends when the browser closes the native messaging
    connection or an incomplete message is received.
    """

    while True:

        # Read the next message from Chrome.
        message = read_message()

        # Stop when there is no complete message available.
        if message is None:
            break

        try:
            # Validate and forward the message to the desktop app,
            # then send the result back to Chrome.
            write_message(post_to_desktop(message))

        except Exception as exc:
            # Return errors as JSON instead of crashing silently.
            write_message({
                "ok": False,
                "error": str(exc)
            })


# ============================================================
# Python Entry Point
# ============================================================
# Only execute main() when this file is run directly.
#
# This prevents main() from running automatically if the file
# is imported as a Python module.
# ============================================================
if __name__ == "__main__":
    main()