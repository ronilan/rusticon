@echo off

set PLATFORM=%1
shift

if "%PLATFORM%"=="--terminal" (
    cargo run --bin rusticon -- %*
)

if "%PLATFORM%"=="--macos" (
    cargo run --bin rusticon_macos --features macos-native -- %*
)

if "%PLATFORM%"=="--windows" (
    cargo run --bin rusticon_windows --features windows-native -- %*
)

if "%PLATFORM%"=="--wasm" (
    pnpm run dev -- %*
)