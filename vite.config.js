import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import process from "node:process";
import path from "node:path";
import { fileURLToPath } from "node:url";

const host = process.env.TAURI_DEV_HOST;

export default defineConfig(() => ({
  plugins: [vue()],
  resolve: {
    alias: {
      "@": fileURLToPath(new URL("./src", import.meta.url)),
    },
  },
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: host || "127.0.0.1",
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
}));