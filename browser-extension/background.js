const NATIVE_HOST = "com.charliemj.videodownloader";
const recent = new Map();
const TAB_TTL = 20_000;

function safeUrl(url) {
  try {
    const parsed = new URL(url);
    return parsed.protocol === "http:" || parsed.protocol === "https:" ? parsed : null;
  } catch {
    return null;
  }
}

function classify(url, responseHeaders = []) {
  const u = safeUrl(url);
  if (!u) return "unknown";

  const mimeHeader = responseHeaders.find(h => h.name.toLowerCase() === "content-type");
  const mime = (mimeHeader?.value || "").split(";")[0].toLowerCase();

  if (mime.startsWith("video/")) return "video";
  if (mime.startsWith("audio/")) return "audio";

  if (u.pathname.includes("videoplayback")) {
    if (u.searchParams.get("mime")?.startsWith("video/")) return "video";
    if (u.searchParams.get("mime")?.startsWith("audio/")) return "audio";
  }

  if (u.pathname.match(/\.(mp4|webm|mkv|mov|m4v)$/i)) return "video";
  if (u.pathname.match(/\.(mp3|m4a|aac|opus|wav|flac)$/i)) return "audio";
  return "unknown";
}

function normalizeDriveUrl(raw) {
  const u = safeUrl(raw);
  if (!u) return raw;

  if (u.hostname.includes("googlevideo.com") && u.pathname.includes("videoplayback")) {
    ["range", "rn", "rbuf", "ump", "srfvp"].forEach(k => u.searchParams.delete(k));
  }

  return u.toString();
}

function streamFrom(details, type, responseHeaders = []) {
  const url = normalizeDriveUrl(details.url);
  const parsed = safeUrl(url);
  if (!parsed) return null;

  const mime = responseHeaders.find(h => h.name.toLowerCase() === "content-type")?.value?.split(";")[0];

  const quality = parsed.searchParams.get("size") ||
    parsed.searchParams.get("quality") ||
    parsed.searchParams.get("itag") ||
    undefined;

  const size = Number(parsed.searchParams.get("clen")) || undefined;
  const duration = Number(parsed.searchParams.get("dur")) || undefined;

  return {
    id: `${details.tabId}:${type}:${url}`,
    streamType: type,
    tabId: details.tabId,
    url,
    mime,
    quality,
    size,
    duration
  };
}

function emitCapture(tabId, stream) {
  if (tabId < 0 || !stream) return;

  const current = recent.get(tabId) || {
    source: "browser",
    tabId,
    pageUrl: undefined,
    title: undefined,
    streams: new Map(),
    timer: null
  };

  current.streams.set(stream.id, stream);
  recent.set(tabId, current);

  if (current.timer) clearTimeout(current.timer);

  current.timer = setTimeout(() => {
    const streams = [...current.streams.values()];
    if (!streams.length) return;

    chrome.tabs.get(tabId).then(tab => {
      sendNative({
        type: "media_detected",
        source: streams.some(x => x.url.includes("googlevideo.com")) ? "google_drive" : "browser",
        pageUrl: tab?.url,
        title: tab?.title,
        streams
      });
    }).catch(() => {});
  }, 350);
}

function sendNative(message) {
  chrome.runtime.sendNativeMessage(NATIVE_HOST, message, () => {
    if (chrome.runtime.lastError) {
      console.warn("Charlie MJ native host:", chrome.runtime.lastError.message);
    }
  });
}

chrome.runtime.onInstalled.addListener(() => {
  chrome.contextMenus.create({
    id: "charlie-mj-download",
    title: "Download with Charlie MJ",
    contexts: ["page", "link", "video", "audio"]
  });
});

chrome.contextMenus.onClicked.addListener((info, tab) => {
  const url = info.linkUrl || info.srcUrl || info.pageUrl || tab?.url;
  if (url) sendNative({ type: "open_url", url });
});

chrome.action.onClicked.addListener(tab => {
  if (tab?.url) sendNative({ type: "open_url", url: tab.url });
});

chrome.webRequest.onCompleted.addListener(
  details => {
    const type = classify(details.url, details.responseHeaders || []);
    if (type !== "video" && type !== "audio") return;

    const stream = streamFrom(details, type, details.responseHeaders || []);
    emitCapture(details.tabId, stream);
  },
  { urls: ["<all_urls>"] },
  ["responseHeaders"]
);
