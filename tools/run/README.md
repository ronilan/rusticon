# run — Application Target Runner Tool

> The run tool is a Rust binary that provides both a **headless CLI** mode and an **interactive TUI** select menu for launching your application across different target platforms.

## Overview

This tool checks command-line arguments to launch your application target (Terminal CLI, Web/WASM, or OS-native GUI window). If no parameter (or an invalid parameter) is provided, it opens a compact terminal UI built with [Incredible](https://github.com/ronilan/incredible-alpha) featuring a rounded selection menu aligned to the left.

The tool is defined in the root `Cargo.toml` as:

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
./run --wasm            # Builds WASM and serves it at http://localhost:4627
./run --macos           # Launches macOS native GUI binary via cargo run (macOS only)
./run --windows         # Launches Windows native GUI binary via cargo run (Windows only)
```

Target short names are also supported (e.g., `t`, `web`, `mac`, `win`). Any extra arguments passed after the target are forwarded directly to the underlying command. An explicit `--` separator is allowed: `./run terminal -- foo.txt`.

```bash
./run terminal foo.txt       # forwards foo.txt to the terminal binary
./run terminal -- foo.txt    # same, explicit separator
```

### Interactive TUI mode

Omit target parameters (or pass an unknown parameter) to open the interactive select menu:

```bash
# Opens interactive TUI menu
./run
```

- Navigate through options using Arrow keys.
- Press **Enter** or **Left Mouse Click** on a target to execute it with the shown args.
- The **Args** label below the list shows extra args forwarded to the target. Prefill it from the command line: `./run foo.txt` opens the menu with `foo.txt` shown — just pick Terminal and press **Enter**. Args with spaces can be quoted: `./run "my file.txt"`.
- `Escape` exits without running anything.

## Options & OS Gating

The interactive select menu automatically adapts to your operating system:

| Option | Command Executed | Availability |
|--------|------------------|--------------|
| **Terminal (CLI)** | `cargo run --bin <package_name>` | All platforms |
| **Web (WASM)** | `./package wasm --preview` (build + static serve at http://localhost:4627) | All platforms |
| **macOS Native** | `cargo run --bin <package_name>_macos --features macos-native` | macOS only |
| **Windows Native** | `cargo run --bin <package_name>_windows --features windows-native` | Windows only |

## Building & Copying the binary

To build the release binary and copy it to the root directory as `./run` (or `./run.exe` on Windows):

```bash
cargo build --release --bin run
```

Then copy it to the project root (see the root [README Build Tools](../../README.md) section for macOS and Windows copy commands).