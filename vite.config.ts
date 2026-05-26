import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";
import { cargoMetadata } from "./scripts/vite-plugin-cargo-metadata";

export default defineConfig({
  root: "web",
  base: "./",
  // Removed topLevelAwait() completely to fix the Rollup build error
  plugins: [wasm(), cargoMetadata()],
  server: {
    host: "0.0.0.0",
    port: 5173,
    fs: {
      allow: [".."],
    },
  },
  build: {
    // "esnext" instructs Vite to preserve native top-level await out-of-the-box
    target: "esnext", 
    outDir: "../docs",
    emptyOutDir: true,
  },
});
