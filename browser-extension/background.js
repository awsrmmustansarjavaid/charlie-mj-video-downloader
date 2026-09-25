// ============================================================
// Charlie MJ Video Downloader - Chrome Extension Background
// ============================================================
// Purpose:
// - Detect video and audio media requests from browser tabs
// - Identify media URLs and their MIME types
// - Collect detected streams per browser tab
// - Send detected media information to the native host
// - Allow users to send page/media URLs to Charlie MJ
//
// Architecture:
//
// Chrome Browser
//      ↓
// Chrome Extension Background Script
//      ↓
// Native Messaging Host
//      ↓
// Charlie MJ Video Downloader Desktop App
// ============================================================


// ------------------------------------------------------------
// Native Messaging Host Name
// ------------------------------------------------------------
// This must match the native messaging host name registered
// with Chrome.
//
// The native host receives messages from this extension and
// forwards them to the Charlie MJ desktop application.
const NATIVE_HOST = "com.charliemj.videodownloader";


// ------------------------------------------------------------
// Recently Detected Media
// ------------------------------------------------------------
// Map of browser tab IDs to recently detected media information.
//
// Key:
//   tabId
//
// Value:
//   Information about the page and detected streams.
const recent = new Map();


// ------------------------------------------------------------
// Tab Detection Lifetime
// ------------------------------------------------------------
// Defines how long detected information can be considered
// recent.
//
// 20,000 milliseconds = 20 seconds.
//
// This value can be used for controlling how long media
// detection information is kept for a tab.
const TAB_TTL = 20_000;


// ============================================================
// Validate and Parse URL
// ============================================================
// Makes sure that the supplied value is a valid HTTP or HTTPS
// URL.
//
// Returns:
//   URL object  → when the URL is valid
//   null        → when the URL is invalid or unsupported
//
// Only HTTP and HTTPS URLs are accepted because the downloader
// communicates with normal web resources.
function safeUrl(url) {
  try {
    // Convert the string into a URL object.
    const parsed = new URL(url);

    // Allow only HTTP and HTTPS protocols.
    return parsed.protocol === "http:" || parsed.protocol === "https:"
      ? parsed
      : null;
  } catch {
    // URL parsing failed.
    return null;
  }
}


// ============================================================
// Detect Media Type
// ============================================================
// Attempts to determine whether a network request contains:
//
// - video
// - audio
// - unknown
//
// Detection is performed using several methods:
//
// 1. HTTP Content-Type response header
// 2. Google video playback URL parameters
// 3. File extension in the URL
//
// This makes media detection more reliable across different
// websites and streaming systems.
function classify(url, responseHeaders = []) {

  // Validate and parse the URL first.
  const u = safeUrl(url);

  // Unsupported or invalid URLs cannot be classified.
  if (!u) return "unknown";


  // ----------------------------------------------------------
  // Check Content-Type Header
  // ----------------------------------------------------------
  // Search the response headers for Content-Type.
  const mimeHeader = responseHeaders.find(
    h => h.name.toLowerCase() === "content-type"
  );

  // Remove parameters such as:
  // video/mp4; codecs=...
  //
  // Leaving only:
  // video/mp4
  const mime = (mimeHeader?.value || "")
    .split(";")[0]
    .toLowerCase();


  // Content-Type explicitly identifies a video.
  if (mime.startsWith("video/")) return "video";

  // Content-Type explicitly identifies an audio stream.
  if (mime.startsWith("audio/")) return "audio";


  // ----------------------------------------------------------
  // Check Google Video Playback URLs
  // ----------------------------------------------------------
  // Some streaming services use URLs containing
  // "videoplayback".
  //
  // The "mime" query parameter can identify the actual stream
  // type when the HTTP Content-Type header is not enough.
  if (u.pathname.includes("videoplayback")) {

    // Example:
    // ?mime=video/mp4
    if (u.searchParams.get("mime")?.startsWith("video/")) {
      return "video";
    }

    // Example:
    // ?mime=audio/mp4
    if (u.searchParams.get("mime")?.startsWith("audio/")) {
      return "audio";
    }
  }


  // ----------------------------------------------------------
  // Check Common Video File Extensions
  // ----------------------------------------------------------
  // Detect video based on the URL path.
  //
  // Supported examples:
  // .mp4
  // .webm
  // .mkv
  // .mov
  // .m4v
  if (u.pathname.match(/\.(mp4|webm|mkv|mov|m4v)$/i)) {
    return "video";
  }


  // ----------------------------------------------------------
  // Check Common Audio File Extensions
  // ----------------------------------------------------------
  // Detect audio based on the URL path.
  //
  // Supported examples:
  // .mp3
  // .m4a
  // .aac
  // .opus
  // .wav
  // .flac
  if (u.pathname.match(/\.(mp3|m4a|aac|opus|wav|flac)$/i)) {
    return "audio";
  }


  // The request does not appear to contain supported media.
  return "unknown";
}


// ============================================================
// Normalize Google Drive / Google Video URL
// ============================================================
// Cleans certain temporary query parameters from Google video
// playback URLs.
//
// Some parameters can change between requests even though the
// underlying media stream is the same.
//
// Removing these parameters helps prevent duplicate stream
// entries from being created.
function normalizeDriveUrl(raw) {
  // Keep the complete browser-issued media URL intact.
  // Google video playback URLs can contain signed/temporary
  // parameters; removing them can invalidate the URL and cause
  // a direct browser-capture download to fail.
  return raw;
}


// ============================================================
// Create Stream Information Object
// ============================================================
// Converts a detected web request into a normalized stream
// object.
//
// The resulting object contains information needed by the
// Charlie MJ desktop downloader.
//
// Includes:
// - stream type
// - tab ID
// - media URL
// - MIME type
// - quality information
// - file size
// - duration
function streamFrom(details, type, responseHeaders = []) {

  // Normalize the detected media URL.
  const url = normalizeDriveUrl(details.url);

  // Parse the normalized URL.
  const parsed = safeUrl(url);

  // Stop if the URL is invalid.
  if (!parsed) return null;


  // ----------------------------------------------------------
  // Read MIME Type
  // ----------------------------------------------------------
  // Get Content-Type from the response headers.
  const mime = responseHeaders.find(
    h => h.name.toLowerCase() === "content-type"
  )?.value?.split(";")[0];


  // ----------------------------------------------------------
  // Detect Quality Information
  // ----------------------------------------------------------
  // Some media URLs expose quality-related information through
  // query parameters.
  //
  // Possible values:
  // - size
  // - quality
  // - itag
  //
  // The first available value is used.
  const quality =
    parsed.searchParams.get("size") ||
    parsed.searchParams.get("quality") ||
    parsed.searchParams.get("itag") ||
    undefined;


  // ----------------------------------------------------------
  // Detect Media Size
  // ----------------------------------------------------------
  // "clen" can contain the content length in bytes.
  //
  // Number(...) converts the string into a JavaScript number.
  // If the value is missing or invalid, undefined is returned.
  const size =
    Number(parsed.searchParams.get("clen")) || undefined;


  // ----------------------------------------------------------
  // Detect Media Duration
  // ----------------------------------------------------------
  // "dur" can contain the media duration in seconds.
  const duration =
    Number(parsed.searchParams.get("dur")) || undefined;


  // ----------------------------------------------------------
  // Build Stream Object
  // ----------------------------------------------------------
  return {
    // Unique identifier for this stream.
    //
    // Combining tab ID, stream type, and URL helps prevent
    // duplicate stream entries.
    id: `${details.tabId}:${type}:${url}`,

    // "video" or "audio".
    streamType: type,

    // Browser tab where the stream was detected.
    tabId: details.tabId,

    // Actual media URL.
    url,

    // Media MIME type, if available.
    mime,

    // Quality/itag information, if available.
    quality,

    // Media size in bytes, if available.
    size,

    // Media duration in seconds, if available.
    duration
  };
}


// ============================================================
// Store and Emit Detected Media
// ============================================================
// Stores detected streams for a specific browser tab.
//
// Multiple network requests can happen very quickly while a
// video is loading. Instead of immediately sending every
// request to the desktop application, streams are collected
// briefly and then sent together.
//
// This reduces duplicate messages and unnecessary communication.
function emitCapture(tabId, stream) {

  // Ignore invalid browser tab IDs or missing streams.
  if (tabId < 0 || !stream) return;


  // ----------------------------------------------------------
  // Get Existing Tab Information
  // ----------------------------------------------------------
  // If this tab has no previous detection data, create a new
  // record.
  const current = recent.get(tabId) || {
    // Default source is the browser.
    source: "browser",

    // Store the browser tab ID.
    tabId,

    // Page URL will be retrieved later.
    pageUrl: undefined,

    // Page title will be retrieved later.
    title: undefined,

    // Store streams in a Map to avoid duplicates.
    streams: new Map(),

    // Timer used to group multiple detections.
    timer: null
  };


  // ----------------------------------------------------------
  // Store the Detected Stream
  // ----------------------------------------------------------
  // Using the stream ID as the Map key prevents the same stream
  // from being stored repeatedly.
  current.streams.set(stream.id, stream);

  // Save the updated tab information.
  recent.set(tabId, current);


  // ----------------------------------------------------------
  // Reset Existing Timer
  // ----------------------------------------------------------
  // If another media request was detected recently, cancel the
  // previous timer.
  //
  // A new timer will be created below so that multiple media
  // requests can be grouped together.
  if (current.timer) clearTimeout(current.timer);


  // ----------------------------------------------------------
  // Wait Briefly Before Sending
  // ----------------------------------------------------------
  // Wait 350 milliseconds before sending the collected streams.
  //
  // This gives the browser time to detect additional streams
  // belonging to the same media session.
  current.timer = setTimeout(() => {

    // Convert the Map into a normal array of stream objects.
    const streams = [...current.streams.values()];


    // Do not send anything if no streams are available.
    if (!streams.length) return;


    // --------------------------------------------------------
    // Get Current Browser Tab Information
    // --------------------------------------------------------
    // Retrieve the current page URL and title so the desktop
    // application knows where the media was detected.
    chrome.tabs.get(tabId).then(tab => {

      current.pageUrl = tab?.url;
      current.title = tab?.title;

      // ------------------------------------------------------
      // Send Media Detection Message
      // ------------------------------------------------------
      sendNative({
        // Message type understood by the native host.
        type: "media_detected",

        // If any detected URL belongs to googlevideo.com,
        // identify the source as Google Drive.
        //
        // Otherwise identify it as a normal browser source.
        source: streams.some(
          x => x.url.includes("googlevideo.com")
        )
          ? "google_drive"
          : "browser",

        // Current page URL.
        pageUrl: tab?.url,

        // Current page title.
        title: tab?.title,

        // All collected media streams.
        streams
      });

    }).catch(() => {
      // Ignore tab lookup errors.
      //
      // The tab may have been closed before information could
      // be retrieved.
    });

  }, 350);
}


// ============================================================
// Send Message to Native Host
// ============================================================
// Sends a message from the Chrome extension to the registered
// Chrome Native Messaging host.
//
// The native host is responsible for forwarding the message to
// the Charlie MJ desktop application.
function sendNative(message) {

  chrome.runtime.sendNativeMessage(
    NATIVE_HOST,
    message,
    () => {

      // Check whether Chrome reported a Native Messaging error.
      if (chrome.runtime.lastError) {

        // Log the error for debugging.
        console.warn(
          "Charlie MJ native host:",
          chrome.runtime.lastError.message
        );
      }
    }
  );
}



// ============================================================
// Content-script overlay bridge
// ============================================================
chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
  if (message?.type !== "get_capture" && message?.type !== "send_capture") {
    return false;
  }

  const tabId = sender.tab?.id;
  if (typeof tabId !== "number") {
    sendResponse({ ok: false, error: "No browser tab is available." });
    return false;
  }

  const current = recent.get(tabId);
  const streams = current ? [...current.streams.values()] : [];

  if (message.type === "get_capture") {
    sendResponse({
      source: current?.source || "browser",
      tabId,
      pageUrl: current?.pageUrl || sender.tab?.url,
      title: current?.title || sender.tab?.title,
      streams
    });
    return false;
  }

  if (!streams.length) {
    sendResponse({ ok: false, error: "No supported media has been detected yet." });
    return false;
  }

  sendNative({
    type: "media_detected",
    source: streams.some(x => x.url.includes("googlevideo.com"))
      ? "google_drive"
      : "browser",
    pageUrl: sender.tab?.url,
    title: sender.tab?.title,
    streams
  });

  sendResponse({ ok: true });
  return false;
});

// ============================================================
// Extension Installation Handler
// ============================================================
// Runs when the Chrome extension is installed or updated.
//
// Creates the context-menu option:
// "Download with Charlie MJ"
chrome.runtime.onInstalled.addListener(() => {

  chrome.contextMenus.create({
    // Internal context-menu item ID.
    id: "charlie-mj-download",

    // Text displayed to the user.
    title: "Download with Charlie MJ",

    // Contexts where the menu item should appear.
    //
    // page  → current webpage
    // link  → links
    // video → video elements
    // audio → audio elements
    contexts: ["page", "link", "video", "audio"]
  });
});


// ============================================================
// Context Menu Click Handler
// ============================================================
// Handles clicks on "Download with Charlie MJ".
//
// Depending on the clicked item, the URL may come from:
//
// - linkUrl
// - srcUrl
// - pageUrl
// - current tab URL
chrome.contextMenus.onClicked.addListener((info, tab) => {

  // Select the most relevant URL available.
  const url =
    info.linkUrl ||
    info.srcUrl ||
    info.pageUrl ||
    tab?.url;


  // Send the URL to the native host when available.
  if (url) {
    sendNative({
      type: "open_url",
      url
    });
  }
});


// ============================================================
// Toolbar Button Click Handler
// ============================================================
// Runs when the user clicks the Chrome extension action icon.
//
// Sends the current tab URL to the Charlie MJ desktop
// application.
chrome.action.onClicked.addListener(tab => {

  // Make sure the tab has a URL.
  if (tab?.url) {

    // Tell the native host to open/process the URL.
    sendNative({
      type: "open_url",
      url: tab.url
    });
  }
});


// ============================================================
// Monitor Completed Network Requests
// ============================================================
// Chrome's webRequest API allows the extension to inspect
// completed network requests.
//
// The extension uses this to identify video and audio streams
// requested by webpages.
chrome.webRequest.onCompleted.addListener(

  // ----------------------------------------------------------
  // Network Request Handler
  // ----------------------------------------------------------
  details => {

    // Determine whether the request contains video or audio.
    const type = classify(
      details.url,
      details.responseHeaders || []
    );


    // Ignore requests that are not supported media.
    if (type !== "video" && type !== "audio") return;


    // Convert the detected request into a normalized stream
    // object.
    const stream = streamFrom(
      details,
      type,
      details.responseHeaders || []
    );


    // Store the stream and eventually send it to the native
    // messaging host.
    emitCapture(details.tabId, stream);
  },


  // ----------------------------------------------------------
  // Request Filter
  // ----------------------------------------------------------
  // Monitor requests from all URLs.
  { urls: ["<all_urls>"] },


  // ----------------------------------------------------------
  // Extra Information
  // ----------------------------------------------------------
  // Request response headers so classify() and streamFrom()
  // can inspect the Content-Type header.
  ["responseHeaders"]
);