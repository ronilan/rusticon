#[cfg(feature = "macos-native")]
mod core;
#[cfg(feature = "macos-native")]
mod features;
#[cfg(feature = "macos-native")]
mod platform;
#[cfg(feature = "macos-native")]
mod runtime;
#[cfg(feature = "macos-native")]
mod screens;
#[cfg(feature = "macos-native")]
mod ui;

#[cfg(feature = "macos-native")]
pub use crate::core::io::RusticonIo;
#[cfg(feature = "macos-native")]
pub use crate::core::model::{AppPhase, MIN_SPLASH_MS, State};

#[cfg(feature = "macos-native")]
fn main() {
    incredible_window_macos::set_window_title(
        option_env!("APP_NAME").unwrap_or("An Incredible App"),
    );
    platform::init();
    incredible_window_macos::run_app(runtime::run);
}

#[cfg(not(feature = "macos-native"))]
fn main() {
    eprintln!(
        "This binary is only available for macOS targets. Build with --features macos-native."
    );
}
