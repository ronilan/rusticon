#![cfg_attr(
    all(feature = "windows-native", not(debug_assertions)),
    windows_subsystem = "windows"
)]

#[cfg(feature = "windows-native")]
mod app;
#[cfg(feature = "windows-native")]
mod core;
#[cfg(feature = "windows-native")]
mod features;
#[cfg(feature = "windows-native")]
mod platform;
#[cfg(feature = "windows-native")]
mod runtime;
#[cfg(feature = "windows-native")]
mod screens;
#[cfg(feature = "windows-native")]
mod ui;

pub use crate::core::io::RusticonIo;
#[cfg(feature = "windows-native")]
pub use crate::core::model::{AppPhase, MIN_SPLASH_MS, State};

#[cfg(feature = "windows-native")]
fn main() {
    incredible_window_windows::set_window_title("Rusticon");
    platform::init();
    incredible_window_windows::run_app(runtime::run);
}

#[cfg(not(feature = "windows-native"))]
fn main() {
    eprintln!(
        "This binary is only available for Windows targets. Build with --features windows-native."
    );
}
