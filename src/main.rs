#[cfg(not(target_arch = "wasm32"))]
mod core;
#[cfg(not(target_arch = "wasm32"))]
mod features;
#[cfg(not(target_arch = "wasm32"))]
mod platform;
#[cfg(not(target_arch = "wasm32"))]
mod rusticon_screen;
#[cfg(not(target_arch = "wasm32"))]
mod splash_screen;
#[cfg(not(target_arch = "wasm32"))]
mod ui;

#[cfg(not(target_arch = "wasm32"))]
pub use core::model::{SplashState, State, MIN_SPLASH_LOOPS};

#[cfg(not(target_arch = "wasm32"))]
use platform::native_io::NativeIo;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let io = NativeIo::new();
    core::application_flow::run_flow(&io);
}

#[cfg(target_arch = "wasm32")]
fn main() {}
