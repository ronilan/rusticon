# Development Environment Prerequisites

This document outlines the system requirements and installation steps needed to develop with this template.

## Stack

* Rust
* Cargo
* Incredible
* WebAssembly
* Pnpm
* Vite
* macOS AppKit (via objc2)
* Windows API (via windows-rs)

## macOS

1. **Install Rust** - In Terminal, run the command from [rustup.rs](https://rustup.rs/):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. **Install Xcode Command Line Tools**:
   ```bash
   xcode-select --install
   ```
3. **Enable pnpm**:
   ```bash
   corepack enable
   ```
4. **Install wasm-pack**:
   ```bash
   cargo install wasm-pack
   ```
5. **Install librsvg** (for macOS app icon generation):
   ```bash
   brew install librsvg
   ```

## Windows

1. **Install Rust** - Download and run [rustup-init.exe](https://rustup.rs/) from [rustup.rs](https://rustup.rs/)
2. **Enable pnpm** - Open PowerShell and run:
   ```powershell
   corepack enable
   ```
3. **Install wasm-pack** - In PowerShell:
   ```powershell
   cargo install wasm-pack
   ```
4. **Install Visual Studio Build Tools** - Download from [visualstudio.microsoft.com](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022). In the installer, select the **"Desktop development with C++"** workload. This is required to compile the `sharp` native image module and provides the Windows SDK.
