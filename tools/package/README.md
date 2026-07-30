# package — Application Packaging & Task Runner Tool

> The package tool is a Rust binary that provides both a **headless CLI** mode and an **interactive TUI** menu for orchestrating project build, bundle, and publish tasks across supported targets.

## Overview

This tool allows you to select a packaging target (All, Terminal, Web, macOS Native) and configure task flags (Clean, Bundle, Publish, Preview site).

It is defined in the workspace `Cargo.toml` as:

```toml
[[bin]]
name = "package"
path = "tools/package/main.rs"
```

## Modes

### Headless mode

Provide a target parameter and optional flag arguments:

```bash
./package --all            # Packages all targets
./package --terminal       # Packages terminal CLI target
./package --wasm           # Builds WASM + site (production)
./package --macos          # Packages macOS native binary
./package --clean --all    # Cleans build output and packages all
```

### Interactive TUI mode

Omit target parameters to open the interactive selection menu:

```bash
# Opens interactive TUI menu
./package
```

- Navigate and select options using Arrow keys and Enter/Space.
- Press **Enter** or **Left Mouse Click** to start execution.

## Options & Flags

| Flag | Description |
|------|-------------|
| `--clean` | Runs clean script (`node scripts/clean.js`) before packaging |
| `--preview` | After WASM build completes, runs `npx vite preview` to serve the site |
| `--bundle` | Executes platform bundle script (`bundle.js` / `bundle_windows.js`) |
| `--publish` | Runs publish script (`node scripts/publish.js`) after packaging |

## Building & Copying the binary

To build the release binary and copy it to the root directory as `./package` (or `./package.exe` on Windows):

```bash
pnpm run build:package
```

This runs `cargo build --release --bin package` and executes `tools/copy-package.js`.
