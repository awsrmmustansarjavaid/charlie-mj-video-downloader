(() => {
  const ROOT_ID = "charlie-mj-download-overlay";
  if (window.top !== window) return;

  const root = document.createElement("div");
  root.id = ROOT_ID;
  root.innerHTML = `
    <button id="cmj-toggle" title="Charlie MJ Video Downloader">⬇ Charlie MJ</button>
    <section id="cmj-panel" hidden>
      <header>
        <strong>Charlie MJ</strong>
        <button id="cmj-close" aria-label="Close">×</button>
      </header>
      <div id="cmj-status">Watching for authorized media…</div>
      <div id="cmj-streams"></div>
      <button id="cmj-send" disabled>Send detected media to Charlie MJ</button>
    </section>
  `;
  document.documentElement.appendChild(root);

  const toggle = root.querySelector("#cmj-toggle");
  const panel = root.querySelector("#cmj-panel");
  const close = root.querySelector("#cmj-close");
  const status = root.querySelector("#cmj-status");
  const streamsEl = root.querySelector("#cmj-streams");
  const send = root.querySelector("#cmj-send");

  let capture = null;

  function render(data) {
    capture = data;
    const streams = data?.streams || [];
    const videos = streams.filter(s => s.streamType === "video");
    const audios = streams.filter(s => s.streamType === "audio");

    status.textContent = videos.length || audios.length
      ? `${videos.length} video stream(s) · ${audios.length} audio stream(s)`
      : "No supported media detected yet.";

    streamsEl.innerHTML = streams.slice(0, 12).map(s => {
      const label = s.quality || s.mime || s.streamType;
      return `<div class="cmj-stream"><span>${s.streamType}</span><span>${label}</span></div>`;
    }).join("");

    send.disabled = streams.length === 0;
  }

  function refresh() {
    chrome.runtime.sendMessage({ type: "get_capture" }, data => {
      if (chrome.runtime.lastError) {
        status.textContent = "Charlie MJ background service is unavailable.";
        return;
      }
      render(data);
    });
  }

  toggle.addEventListener("click", () => {
    panel.hidden = !panel.hidden;
    if (!panel.hidden) refresh();
  });

  close.addEventListener("click", () => {
    panel.hidden = true;
  });

  send.addEventListener("click", () => {
    send.disabled = true;
    status.textContent = "Sending detected media to Charlie MJ…";
    chrome.runtime.sendMessage({ type: "send_capture" }, result => {
      if (chrome.runtime.lastError || !result?.ok) {
        status.textContent = result?.error || "Could not contact Charlie MJ.";
        send.disabled = false;
        return;
      }
      status.textContent = "Sent. Open Charlie MJ to choose Download & Combine.";
      send.disabled = false;
    });
  });

  // Refresh while the panel is open. Network streams often arrive
  // after the page itself has finished loading.
  setInterval(() => {
    if (!panel.hidden) refresh();
  }, 1500);
})();
