// ============================================================
// Charlie MJ Video Downloader - IPC Server
// ============================================================
// Purpose:
// - Receives messages from the Chrome Native Messaging Host.
// - Runs a small local HTTP server on 127.0.0.1:47821.
// - Parses browser media detection messages.
// - Stores captured media information in application state.
// - Sends captured media events to the Tauri frontend.
//
// Communication flow:
//
// Chrome Extension
//       ↓
// Chrome Native Messaging
//       ↓
// native-host/host.py
//       ↓ HTTP POST
// 127.0.0.1:47821
//       ↓
// ipc.rs
//       ↓
// browser_capture.rs
//       ↓
// AppState + Tauri Event
//       ↓
// React Frontend
// ============================================================

// Browser media message parser and shared application state.
use crate::{browser_capture, state::AppState};

// Used to represent incoming JSON data.
use serde_json::Value;

// Tauri application handle, event emitter, and state manager.
use tauri::{AppHandle, Emitter, Manager};

// Tokio networking and asynchronous I/O utilities.
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

// ------------------------------------------------------------
// Start Local IPC Server
// ------------------------------------------------------------
// Starts the local HTTP server used by the Chrome Native
// Messaging Host to communicate with the Tauri application.
//
// The server listens only on localhost:
//
// 127.0.0.1:47821
//
// This means the IPC endpoint is intended for communication
// between applications running on the same Windows machine.
pub async fn start(app: AppHandle) -> Result<(), String> {

    // Bind a TCP listener to the local IPC port.
    //
    // TcpListener accepts incoming TCP connections from the
    // Chrome Native Messaging Host.
    let listener = TcpListener::bind("127.0.0.1:47821")
        .await
        .map_err(|e| e.to_string())?;

    // --------------------------------------------------------
    // Accept Incoming Connections
    // --------------------------------------------------------
    // Keep the IPC server running continuously and wait for
    // new connections from the native messaging host.
    loop {

        // Accept the next incoming TCP connection.
        //
        // `socket` is the connection used to read the HTTP
        // request and send the HTTP response.
        let (mut socket, _) = listener
            .accept()
            .await
            .map_err(|e| e.to_string())?;

        // Clone the Tauri application handle so the connection
        // can safely use it inside the spawned async task.
        let app = app.clone();

        // ----------------------------------------------------
        // Handle Connection Asynchronously
        // ----------------------------------------------------
        // Each incoming connection gets its own Tokio task.
        //
        // This allows multiple browser media messages to be
        // processed without blocking the main IPC listener.
        tokio::spawn(async move {

            // Allocate a 1 MB buffer for the incoming HTTP
            // request.
            //
            // This is intended to accommodate JSON payloads
            // containing multiple captured media streams.
            let mut buffer = vec![0u8; 1024 * 1024];

            // Read the incoming request from the TCP socket.
            //
            // If reading fails, terminate this connection.
            let Ok(n) = socket.read(&mut buffer).await else {
                return;
            };

            // Convert the received bytes into text.
            //
            // Invalid UTF-8 bytes are replaced rather than
            // causing the entire connection to fail.
            let request = String::from_utf8_lossy(&buffer[..n]);

            // ------------------------------------------------
            // Extract HTTP Request Body
            // ------------------------------------------------
            // HTTP separates its headers from the request body
            // using a blank line:
            //
            // Headers
            // \r\n
            // \r\n
            // Body
            //
            // The JSON sent by host.py is expected to be inside
            // this body.
            let Some(body) = request.split("\r\n\r\n").nth(1) else {
                return;
            };

            // Parse the HTTP body as generic JSON.
            //
            // browser_capture::parse_message() will later
            // validate and convert this JSON into the application's
            // BrowserMediaCapture model.
            let Ok(value) = serde_json::from_str::<Value>(body) else {
                return;
            };

            // ------------------------------------------------
            // Access Shared Application State
            // ------------------------------------------------
            // Retrieve AppState managed by Tauri.
            //
            // This state contains information shared between
            // the backend components and the frontend.
            let state = app.state::<AppState>();

            // ------------------------------------------------
            // Parse Browser Media Capture
            // ------------------------------------------------
            // Convert the incoming JSON message into a structured
            // BrowserMediaCapture object.
            //
            // The parser also validates the message type and
            // processes the captured video/audio streams.
            match browser_capture::parse_message(value) {

                // Successfully parsed browser media capture.
                Ok(capture) => {

                    // ----------------------------------------
                    // Store Capture in Application State
                    // ----------------------------------------
                    // Lock the shared capture collection so the
                    // new capture can be inserted safely.
                    if let Ok(mut captures) = state.captures.lock() {

                        // Add the newest capture to the beginning
                        // of the collection.
                        captures.insert(0, capture.clone());

                        // Keep only the latest 50 captures.
                        //
                        // This prevents the in-memory collection
                        // from growing indefinitely.
                        captures.truncate(50);
                    }

                    // ----------------------------------------
                    // Notify the Tauri Frontend
                    // ----------------------------------------
                    // Emit a Tauri event named:
                    //
                    // "browser-media-detected"
                    //
                    // The React frontend can listen for this event
                    // and immediately display the newly detected
                    // browser media.
                    let _ = app.emit("browser-media-detected", capture);
                }

                // Invalid or unsupported messages are ignored.
                //
                // The current implementation intentionally does
                // not send a detailed error response to the caller.
                Err(_) => {}
            }

            // ------------------------------------------------
            // Send HTTP Response
            // ------------------------------------------------
            // Send a simple JSON response back to the native
            // messaging host.
            //
            // This confirms that the HTTP request was processed.
            let response =
                "HTTP/1.1 200 OK\r\n\
                 Content-Type: application/json\r\n\
                 Content-Length: 11\r\n\
                 Connection: close\r\n\
                 \r\n\
                 {\"ok\":true}";

            // Write the HTTP response to the socket.
            //
            // The result is ignored because the connection is
            // already finished from the server's perspective.
            let _ = socket.write_all(response.as_bytes()).await;
        });
    }
}