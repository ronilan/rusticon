#[cfg(not(target_arch = "wasm32"))]
mod app;

#[cfg(not(target_arch = "wasm32"))]
pub use app::{core, features, platform, rusticon_screen, screens, splash_screen, ui};

#[cfg(not(target_arch = "wasm32"))]
pub use core::model::{SplashState, State, MIN_SPLASH_MS};

#[cfg(not(target_arch = "wasm32"))]
use platform::native_io::NativeIo;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let io = NativeIo::new();
    core::application_flow::run_flow(&io);
}

#[cfg(target_arch = "wasm32")]
fn main() {}
