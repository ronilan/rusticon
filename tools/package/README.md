# package — Application Packaging & Task Runner Tool

> The package tool is a Rust binary that provides both a **headless CLI** mode and an **interactive TUI** menu for orchestrating project build, bundle, and publish tasks across supported targets.

## Overview

This tool allows you to select a packaging target (All, Terminal, Web, macOS Native) and configure task flags (Clean, Bundle, Publish, Preview site).

The tool is defined in the root `Cargo.toml` as:

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
./package --wasm --preview # Builds WASM + serves docs/ at http://localhost:4627
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
| `--clean` | Cleans all build output before packaging |
| `--preview` | After the WASM build completes, serves the built site in `docs/` locally at http://localhost:4627 |
| `--bundle` | Executes platform-specific bundle step to produce the macOS app bundle / Windows standalone EXE |
| `--publish` | Publishes packaged artifacts as a GitHub release after packaging |

## Web target

The `wasm` target needs **wasm-pack** installed (see [development prerequisites](../../markdowns/DEVELOPMENT_PREREQUISITES.md)) and is otherwise pure Rust:

1. Runs `wasm-pack build --target web --release` (all Rust code is compiled to WASM).
2. Assembles `docs/`: copies static assets from `web/`, the browser glue from `src/main.js`, and the compiled `pkg/` output, injecting metadata (`title`, description, keywords, version, mobile min-width/height, CNAME) from `Cargo.toml` into `docs/index.html`.

With `--preview` the tool then serves `docs/` over HTTP at http://localhost:4627. No bundler is involved — `docs/` is deployable as-is (GitHub Pages, any static host).

## Building & Copying the binary

To build the release binary and copy it to the root directory as `./package` (or `./package.exe` on Windows):

```bash
cargo build --release --bin package
```

Then copy it to the project root (see the root [README Build Tools](../../README.md) section for macOS and Windows copy commands).