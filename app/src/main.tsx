// ============================================================
// Charlie MJ Video Downloader - React Entry Point
// ============================================================
// Purpose:
// - Initializes the React application.
// - Finds the root HTML element.
// - Mounts the main App component.
// - Loads the application's global CSS styles.
//
// Application flow:
//
// index.html
//    ↓
// main.tsx
//    ↓
// App.tsx
//    ↓
// React UI
// ============================================================


// Import React.
// React is required for JSX and React application features.
import React from "react";


// Import ReactDOM's client API.
// createRoot() is used to mount a React application into the
// browser DOM.
import ReactDOM from "react-dom/client";


// Import the main application component.
import App from "./App";


// Import global application styles.
// These styles are applied throughout the React interface.
import "./styles.css";


// ============================================================
// Create React Application Root
// ============================================================
// Find the <div id="root"></div> element from index.html
// and create a React root inside it.
//
// The "!" tells TypeScript that the element is expected to
// exist in the HTML document.
// ============================================================
ReactDOM.createRoot(
  document.getElementById("root")!
).render(

  // ----------------------------------------------------------
  // React Strict Mode
  // ----------------------------------------------------------
  // StrictMode enables additional development-time checks.
  //
  // It helps identify potential problems in React components
  // and lifecycle behavior during development.
  <React.StrictMode>

    // Main Charlie MJ Video Downloader application.
    <App />

  </React.StrictMode>
);