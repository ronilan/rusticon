# config — Project Rename Tool

> The config tool is a Rust binary that provides both a **headless CLI** mode and an **interactive TUI** terminal UI for renaming project template references.

## Overview

This tool reads the current template name and metadata from `Cargo.toml`, lets you edit all five fields, and writes the changes back to every file that references the old name (15 source files across the repo).

It is defined in the workspace `Cargo.toml` as:

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

The tool updates all 15 source files that reference the template name, plus 2 files with `description` fields.

### Template-name files (15 total)

1. `Cargo.toml` — 8 occurrences (package name, 3 bin names, metadata title + app_name + binary_name, default-run)
2. `src/main.rs` — 1 (error message string)
3. `src/macos.rs` — 1 (fallback APP_NAME)
4. `src/windows.rs` — 1 (fallback APP_NAME)
5. `src/main.ts` — 1 (WASM import path)
6. `Info.plist` — 2 (CFBundleExecutable, CFBundleName)
7. `run` — 3 (bash cargo run commands)
8. `run.bat` — 3 (Windows batch cargo run commands)
9. `scripts/build_macos.js` — 4 (cargo build + copy paths + log)
10. `scripts/build_terminal.js` — 3 (cargo build + exe names)
11. `scripts/build_windows.js` — 2 (cargo build + exe name)
12. `pkg/package.json` — 6 (npm name + wasm/js/dts filenames + main + types)
13. `docs/index.html` — 1 (HTML `<title>`)
14. `web/index.html` — 1 (HTML `<title>`)
15. `package.json` — 1 (root workspace `"name": "incredible-template"`)

### Description field occurrences (2 total)

| File | Line | Content |
|------|------|---------|
| `Cargo.toml` | 9 | `description = "..."` |
| `docs/index.html` | 8 | `<meta name="description" content="...">` |

> **Note:** `scripts/vite-plugin-cargo-metadata.ts` reads `metadata.description` from `Cargo.toml` and injects it into the built HTML at build time.

## Copying the binary

A helper script at `tools/copy-config.js` copies the built binary from `target/release/config` to the project root:

```bash
node tools/copy-config.js
```

This is useful when you want to run the config tool as `./config` rather than via `cargo run`.
