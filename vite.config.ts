import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import { cargoMetadata } from "./scripts/vite-plugin-cargo-metadata";

export default defineConfig({
  root: "web",
  base: "./",
  plugins: [wasm(), topLevelAwait(), cargoMetadata()],
  server: {
    host: "0.0.0.0",
    port: 5173,
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
