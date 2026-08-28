import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";

// Tailwind v4 is a Vite plugin now — no tailwind.config.js or PostCSS needed.
export default defineConfig({
  plugins: [react(), tailwindcss()],
  // Tauri expects a fixed port and no auto-clearing of the terminal.
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
  },
});
