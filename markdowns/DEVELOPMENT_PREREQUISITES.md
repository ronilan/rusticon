# Development Environment Prerequisites

This document outlines the system requirements and installation steps needed to develop with this template.

## Stack

* Rust
* Cargo
* Incredible
* WebAssembly
* macOS AppKit (via objc2)
* Windows API (via windows-rs)

## GitHub Authentication

This project depends on the private [incredible-alpha](https://github.com/ronilan/incredible-alpha) crate, pulled by Cargo as a git dependency over HTTPS. The first build (`cargo build-tools`) will fail until your machine can read that repo.

The easiest way to authenticate is the [GitHub CLI](https://cli.github.com/) — download and install it from [cli.github.com](https://cli.github.com/) if you don't have it already:

```bash
gh auth login
```

This configures git's credential helper, so `git` (and Cargo fetching through it) can access the private repository without prompting.

Alternatively, create a fine-grained Personal Access Token with read access to `ronilan/incredible-alpha` and store it with your OS credential manager (macOS Keychain / Windows Credential Manager).

## macOS

1. **Install Xcode Command Line Tools**:
   ```bash
   xcode-select --install
   ```
2. **Install Rust** - Download and run the installer from [rustup.rs](https://rustup.rs/)
3. **Install wasm-pack**:
   ```bash
   cargo install wasm-pack
   ```

## Windows

1. **Install Rust** - Download and run [rustup-init.exe](https://rustup.rs/) from [rustup.rs](https://rustup.rs/)
2. **Install wasm-pack** - In PowerShell:
   ```powershell
   cargo install wasm-pack
   ```
3. **Install Visual Studio Build Tools** - Download from [visualstudio.microsoft.com](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022). In the installer, select the **"Desktop development with C++"** workload. This provides the Windows SDK and is required to build the native Windows binary.