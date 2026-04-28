#[cfg(feature = "macos-native")]
mod app;
#[cfg(feature = "macos-native")]
mod platform;
#[cfg(feature = "macos-native")]
mod runtime;
#[cfg(feature = "macos-native")]
mod core;
#[cfg(feature = "macos-native")]
mod features;
#[cfg(feature = "macos-native")]
mod screens;
#[cfg(feature = "macos-native")]
mod ui;

#[cfg(feature = "macos-native")]
pub use crate::core::model::{AppPhase, State, MIN_SPLASH_MS};
pub use crate::core::io::RusticonIo;

#[cfg(feature = "macos-native")]
fn main() {
    little_tui_window_macos::set_window_title("Rusticon");
    platform::init();
    little_tui_window_macos::run_app(runtime::run);
}

#[cfg(not(feature = "macos-native"))]
fn main() {
    eprintln!("This binary is only available for macOS targets. Build with --features macos-native.");
}
