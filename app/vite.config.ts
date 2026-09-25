// ============================================================
// Charlie MJ Video Downloader - Vite Configuration
// ============================================================
// Purpose:
// - Configure Vite for the frontend application
// - Enable React support
// - Configure the development server
// - Keep the terminal output visible during development
//
// Vite is responsible for:
// - Running the frontend development server
// - Processing TypeScript/React files
// - Building the production frontend
// ============================================================


// ------------------------------------------------------------
// Import Vite Configuration Helper
// ------------------------------------------------------------
// defineConfig() provides better TypeScript support and
// configuration validation for the Vite configuration file.
import { defineConfig } from "vite";


// ------------------------------------------------------------
// Import React Vite Plugin
// ------------------------------------------------------------
// @vitejs/plugin-react enables Vite to process React components
// and JSX/TSX files.
//
// This allows the application to use files such as:
// - .jsx
// - .tsx
//
// It also provides React development features such as Fast
// Refresh during development.
import react from "@vitejs/plugin-react";


// ============================================================
// Vite Configuration
// ============================================================
// Export the configuration used by Vite for development
// and production builds.
export default defineConfig({

  // ----------------------------------------------------------
  // Vite Plugins
  // ----------------------------------------------------------
  // Enable the React plugin.
  //
  // This allows Vite to understand and process the React
  // frontend used by Charlie MJ Video Downloader.
  plugins: [
    react()
  ],


  // ----------------------------------------------------------
  // Keep Terminal Output Visible
  // ----------------------------------------------------------
  // Prevent Vite from automatically clearing the terminal
  // screen when displaying development server information.
  //
  // This can make logs and debugging messages easier to see.
  clearScreen: false,


  // ----------------------------------------------------------
  // Development Server Configuration
  // ----------------------------------------------------------
  server: {

    // Port used by the Vite development server.
    //
    // Example:
    // http://localhost:1420
    //
    // Tauri development configuration commonly uses this
    // port for the frontend development server.
    port: 1420,


    // --------------------------------------------------------
    // Strict Port
    // --------------------------------------------------------
    // If port 1420 is already being used, Vite will stop
    // instead of automatically selecting another port.
    //
    // This is useful for Tauri because the desktop application
    // expects the frontend to run on a specific port.
    strictPort: true
  }

});