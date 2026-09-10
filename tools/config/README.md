# config — Project Rename Tool

> The config tool is a Rust binary that provides both a **headless CLI** mode and an **interactive TUI** terminal UI for renaming project template references.

## Overview

This tool reads the current template name and metadata from `Cargo.toml`, lets you edit all five fields, and writes the changes back to every file that references the old name.

The tool is defined in the root `Cargo.toml` as:

```toml
[[bin]]
name = "config"
path = "tools/config/main.rs"
```

## Modes

### Headless mode

Provide all five CLI flags — changes are applied immediately with no UI:

```bash
cargo run --bin config -- \
  --name my_app \
  --app-name "My App" \
  --tagline "My App" \
  --keywords "tui,rust" \
  --description "A minimal My App for terminal, wasm and native mac os targets."
```

### Interactive TUI mode

Omit any of the five flags and the tool opens a terminal UI (built with [Incredible](https://github.com/ronilan/incredible-alpha)) with the current values pre-filled. Tab between fields, edit them, and press **Apply** to write changes.

```bash
# Opens TUI with current values pre-filled
cargo run --bin config
```

## CLI flags

| Flag | Corresponds to | Required in headless mode |
|------|---------------|---------------------------|
| `--name` | Base identifier (snake_case, e.g. `my_app`) | ✅ |
| `--app-name` | Human-readable display name | ✅ |
| `--tagline` | Short tagline for `<title>` | ✅ |
| `--keywords` | Comma-separated keywords | ✅ |
| `--description` | Package description | ✅ |

## Validation rules

| Field | Rule |
|-------|------|
| **App Name** | Required. Any text, no newlines. |
| **Name** | Required. `snake_case`, ASCII only, max 64 chars, starts with a letter. |
| **Tagline** | Optional. No newlines. |
| **Keywords** | Max 5, comma-separated. Each: ASCII, max 20 chars, starts with alphanumeric. |
| **Description** | Plain text, no newlines, max 160 chars recommended. |

## Files affected

The tool updates source files that reference the template name, plus metadata fields.

1. `Cargo.toml` — package name, 3 bin names, metadata title + app_name + binary_name, default-run
2. `Info.plist` — CFBundleExecutable, CFBundleName
3. `src/main.js` — WASM import path
4. `web/index.html` — HTML `<title>`

> **Note:** At build time the web target reads `title`, `description`, `keywords`, `version`, `mobile-min-width`, `mobile-min-height`, and `cname` from `Cargo.toml` and injects them into `docs/index.html` (see [`tools/package/README.md`](../package/README.md#web-target)).

## Building & Copying the binary

To build the release binary and copy it to the project root as `./config` (or `./config.exe` on Windows):

```bash
cargo build --release --bin config
```

Then copy it to the project root (see the root [README Build Tools](../../README.md) section for macOS and Windows copy commands).