use crate::models::{BrowserMediaCapture, CapturedStream};
use chrono::Utc;
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

fn classify_mime(mime: Option<&str>) -> &'static str {
    match mime.unwrap_or_default().split(';').next().unwrap_or_default() {
        x if x.starts_with("video/") => "video",
        x if x.starts_with("audio/") => "audio",
        _ => "unknown",
    }
}

fn normalized_drive_url(raw: &str) -> String {
    let Ok(mut parsed) = url::Url::parse(raw) else {
        return raw.to_string();
    };

    if parsed.host_str().map(|h| h.contains("googlevideo.com")).unwrap_or(false)
        && parsed.path().contains("videoplayback")
    {
        let remove = ["range", "rn", "rbuf", "ump", "srfvp"];
        let pairs: Vec<(String, String)> = parsed.query_pairs()
            .filter(|(k, _)| !remove.contains(&k.as_ref()))
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        parsed.set_query(None);
        {
            let mut q = parsed.query_pairs_mut();
            for (k, v) in pairs {
                q.append_pair(&k, &v);
            }
        }
    }

    parsed.to_string()
}

pub fn parse_message(value: Value) -> Result<BrowserMediaCapture, String> {
    if value.get("type").and_then(Value::as_str) != Some("media_detected") {
        return Err("Unsupported native message type".into());
    }

    let source = value.get("source").and_then(Value::as_str).unwrap_or("browser").to_string();
    let page_url = value.get("pageUrl").and_then(Value::as_str).map(str::to_string);
    let title = value.get("title").and_then(Value::as_str).map(str::to_string);

    let mut dedupe: HashMap<String, CapturedStream> = HashMap::new();

    for item in value.get("streams").and_then(Value::as_array).cloned().unwrap_or_default() {
        let raw_url = item.get("url").and_then(Value::as_str).ok_or("Stream URL missing")?;
        let url = normalized_drive_url(raw_url);

        let mime = item.get("mime").and_then(Value::as_str).map(str::to_string);
        let stream_type = item.get("streamType")
            .and_then(Value::as_str)
            .unwrap_or_else(|| classify_mime(mime.as_deref()))
            .to_string();

        if !matches!(stream_type.as_str(), "video" | "audio") {
            continue;
        }

        let id = Uuid::new_v4().to_string();
        let stream = CapturedStream {
            id: id.clone(),
            source: source.clone(),
            tab_id: item.get("tabId").and_then(Value::as_i64),
            page_url: page_url.clone(),
            title: title.clone(),
            stream_type,
            url,
            mime,
            quality: item.get("quality").and_then(Value::as_str).map(str::to_string),
            width: item.get("width").and_then(Value::as_u64).map(|x| x as u32),
            height: item.get("height").and_then(Value::as_u64).map(|x| x as u32),
            fps: item.get("fps").and_then(Value::as_f64),
            bitrate: item.get("bitrate").and_then(Value::as_u64),
            size: item.get("size").and_then(Value::as_u64),
            duration: item.get("duration").and_then(Value::as_f64),
        };

        let key = format!("{}|{}|{:?}|{:?}", stream.stream_type, stream.url, stream.width, stream.height);
        dedupe.entry(key).or_insert(stream);
    }

    Ok(BrowserMediaCapture {
        capture_id: Uuid::new_v4().to_string(),
        source,
        page_url,
        title,
        streams: dedupe.into_values().collect(),
        created_at: Utc::now().to_rfc3339(),
    })
}
