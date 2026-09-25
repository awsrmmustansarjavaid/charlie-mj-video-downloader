use futures_util::StreamExt;
use reqwest::Client;
use std::{path::{Path, PathBuf}, time::Instant};
use tokio::{fs, io::AsyncWriteExt};

pub async fn download_stream(url: &str, path: &Path) -> Result<u64, String> {
    if !matches!(url::Url::parse(url).map_err(|e| e.to_string())?.scheme(), "http" | "https") {
        return Err("Only HTTP/HTTPS streams are supported".into());
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).await.map_err(|e| e.to_string())?;
    }

    let client = Client::builder()
        .user_agent("Charlie-MJ-Video-Downloader/0.2")
        .build()
        .map_err(|e| e.to_string())?;

    let response = client.get(url).send().await.map_err(|e| e.to_string())?;
    if !response.status().is_success() {
        return Err(format!("Stream request failed: {}", response.status()));
    }

    let mut file = fs::File::create(path).await.map_err(|e| e.to_string())?;
    let mut stream = response.bytes_stream();
    let mut total = 0u64;
    let _started = Instant::now();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| e.to_string())?;
        file.write_all(&chunk).await.map_err(|e| e.to_string())?;
        total += chunk.len() as u64;
    }

    file.flush().await.map_err(|e| e.to_string())?;
    Ok(total)
}

pub fn temp_pair_dir(base: &Path, job_id: &str) -> PathBuf {
    base.join("temp").join(job_id)
}
