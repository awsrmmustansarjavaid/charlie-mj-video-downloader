const NATIVE_HOST = "com.charliemj.videodownloader";

chrome.runtime.onInstalled.addListener(() => {
  chrome.contextMenus.create({
    id: "send-to-charlie-mj",
    title: "Download with Charlie MJ",
    contexts: ["page", "link", "video", "audio"]
  });
});

function send(url) {
  if (!url) return;
  chrome.runtime.sendNativeMessage(
    NATIVE_HOST,
    { type: "open_url", url },
    () => {
      if (chrome.runtime.lastError) {
        console.warn("Charlie MJ native host:", chrome.runtime.lastError.message);
      }
    }
  );
}

chrome.contextMenus.onClicked.addListener((info, tab) => {
  send(info.linkUrl || info.srcUrl || info.pageUrl || tab?.url);
});

chrome.action.onClicked.addListener((tab) => {
  send(tab?.url);
});
