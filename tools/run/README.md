# run — Application Target Runner Tool

> The run tool is a Rust binary that provides both a **headless CLI** mode and an **interactive TUI** select menu for launching your application across different target platforms.

## Overview

This tool checks command-line arguments to launch your application target (Terminal CLI, Web/WASM dev server, or OS-native GUI window). If no parameter (or an invalid parameter) is provided, it opens a compact terminal UI built with [Incredible](https://github.com/ronilan/incredible-alpha) featuring a rounded selection menu aligned to the left.

It is defined in the workspace `Cargo.toml` as:

```toml
[[bin]]
name = "run"
path = "tools/run/main.rs"
```

## Modes

### Headless mode

Provide a target platform parameter to immediately execute the selected target without launching the UI:

```bash
./run --terminal        # Launches terminal CLI binary via cargo run
./run --wasm            # Launches Vite dev server via pnpm run dev
./run --macos           # Launches macOS native GUI binary via cargo run (macOS only)
./run --windows         # Launches Windows native GUI binary via cargo run (Windows only)
```

Target short names are also supported (e.g., `t`, `web`, `mac`, `win`). Any extra arguments passed after the target are forwarded directly to the underlying command.

### Interactive TUI mode

Omit target parameters (or pass an unknown parameter) to open the interactive select menu:

```bash
# Opens interactive TUI menu
./run
```

- Navigate through options using Arrow keys.
- Press **Enter** or **Left Mouse Click** to execute the selected target.

## Options & OS Gating

The interactive select menu automatically adapts to your operating system:

| Option | Command Executed | Availability |
|--------|------------------|--------------|
| **Terminal (CLI)** | `cargo run --bin <package_name>` | All platforms |
| **Web (WASM / Vite)** | `pnpm run dev` | All platforms |
| **macOS Native** | `cargo run --bin <package_name>_macos --features macos-native` | macOS only |
| **Windows Native** | `cargo run --bin <package_name>_windows --features windows-native` | Windows only |

## Building & Copying the binary

To build the release binary and copy it to the root directory as `./run` (or `./run.exe` on Windows):

```bash
pnpm run build:run
```

This runs `cargo build --release --bin run` and executes `tools/copy-run.js`.
