import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

export default defineConfig({
  root: "web",
  base: "./",
  plugins: [wasm(), topLevelAwait()],
  server: {
    host: "0.0.0.0",
    port: 3000,
    fs: {
      allow: [".."],
    },
  },
  build: {
    target: "esnext",
    outDir: "../docs",
    emptyOutDir: true,
  },
});
