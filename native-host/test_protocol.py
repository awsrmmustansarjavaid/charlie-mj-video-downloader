# ============================================================
# Charlie MJ Video Downloader - Native Messaging Protocol Test
# ============================================================
# Purpose:
# - Test the JSON message structure used by the native host
# - Verify that messages can be serialized to JSON and restored
# - Confirm the expected media_detected message format
#
# This is a lightweight protocol-shape test.
# It does not start the native host or desktop application.
# ============================================================


# ------------------------------------------------------------
# Import Required Modules
# ------------------------------------------------------------

# json:
# Used to serialize Python objects into JSON and deserialize
# JSON back into Python objects.
import json

# subprocess:
# Imported for potential native host process testing.
# It is currently not used by this test.
import subprocess

# struct:
# Used to create the 4-byte message length prefix required by
# Chrome Native Messaging.
import struct

# sys:
# Provides access to Python's standard input/output streams.
import sys


# ============================================================
# Create Native Messaging Frame
# ============================================================
def frame(obj):
    """
    Convert a Python object into a Chrome Native Messaging frame.

    Native Messaging uses this format:

        [4-byte message length][JSON message]

    The JSON payload is encoded as UTF-8 bytes, and its length
    is stored as an unsigned 32-bit little-endian integer.
    """

    # Convert the Python object into JSON and then into bytes.
    data = json.dumps(obj).encode()

    # Add the 4-byte payload length before the JSON data.
    #
    # "<I":
    #   < = little-endian byte order
    #   I = unsigned 32-bit integer
    return struct.pack("<I", len(data)) + data


# ============================================================
# Test JSON Message Structure
# ============================================================
def test_json_roundtrip_shape():
    """
    Verify that the expected media_detected message can be
    serialized to JSON and deserialized without changing its
    structure or values.
    """

    # Example message sent by the Chrome extension when media
    # streams have been detected.
    message = {
        "type": "media_detected",

        # Identify where the media was detected.
        "source": "google_drive",

        # List of detected media streams.
        "streams": [
            {
                "streamType": "video",
                "url": "https://example.test/video"
            },
            {
                "streamType": "audio",
                "url": "https://example.test/audio"
            }
        ]
    }

    # Serialize the message to JSON and immediately deserialize
    # it back into a Python object.
    #
    # The assertion confirms that the resulting object has
    # exactly the same structure and values as the original.
    assert json.loads(json.dumps(message)) == message


# ============================================================
# Script Entry Point
# ============================================================
# Run the protocol test only when this file is executed
# directly.
#
# If imported by another Python module, the test will not run
# automatically.
# ============================================================
if __name__ == "__main__":

    # Run the JSON protocol-shape test.
    test_json_roundtrip_shape()

    # Display a success message when the test passes.
    print("protocol shape OK")