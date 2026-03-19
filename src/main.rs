#[cfg(not(target_arch = "wasm32"))]
mod app;

#[cfg(not(target_arch = "wasm32"))]
pub use app::{core, features, platform, screens, ui};

#[cfg(not(target_arch = "wasm32"))]
pub use core::model::{AppPhase, State, MIN_SPLASH_MS};

#[cfg(not(target_arch = "wasm32"))]
use platform::native_io::NativeIo;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let io = NativeIo::new();
    app::app(io);
}

#[cfg(target_arch = "wasm32")]
fn main() {}
