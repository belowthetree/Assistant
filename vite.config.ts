import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import path from "path";

//
const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [vue()],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 5710,
    strictPort: true,
    host: '0.0.0.0',
    cors: false, // 配置 CORS
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
  resolve: {
    alias: {
      '@': path.resolve(__dirname) + "/src", // @ 指向 src 目录
      '~': path.resolve(__dirname), // @ 指向项目根目录
    },
  },
}));
