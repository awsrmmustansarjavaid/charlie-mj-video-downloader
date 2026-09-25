use crate::{browser_capture, state::AppState};
use serde_json::Value;
use tauri::{AppHandle, Emitter, Manager};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

pub async fn start(app: AppHandle) -> Result<(), String> {
    let listener = TcpListener::bind("127.0.0.1:47821")
        .await
        .map_err(|e| e.to_string())?;

    loop {
        let (mut socket, _) = listener.accept().await.map_err(|e| e.to_string())?;
        let app = app.clone();

        tokio::spawn(async move {
            let mut buffer = vec![0u8; 1024 * 1024];
            let Ok(n) = socket.read(&mut buffer).await else { return; };
            let request = String::from_utf8_lossy(&buffer[..n]);

            let Some(body) = request.split("\r\n\r\n").nth(1) else { return; };
            let Ok(value) = serde_json::from_str::<Value>(body) else { return; };

            let state = app.state::<AppState>();

            match browser_capture::parse_message(value) {
                Ok(capture) => {
                    if let Ok(mut captures) = state.captures.lock() {
                        captures.insert(0, capture.clone());
                        captures.truncate(50);
                    }
                    let _ = app.emit("browser-media-detected", capture);
                }
                Err(_) => {}
            }

            let response =
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 11\r\nConnection: close\r\n\r\n{\"ok\":true}";
            let _ = socket.write_all(response.as_bytes()).await;
        });
    }
}
