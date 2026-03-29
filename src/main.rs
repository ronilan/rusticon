#[cfg(not(target_arch = "wasm32"))]
mod app;
#[cfg(not(target_arch = "wasm32"))]
mod platform;
#[cfg(not(target_arch = "wasm32"))]
mod runtime;

#[cfg(not(target_arch = "wasm32"))]
pub use app::{core, features, screens, ui};

#[cfg(not(target_arch = "wasm32"))]
pub use core::model::{AppPhase, State, MIN_SPLASH_MS};

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    platform::init();
    runtime::run(platform::io());
}

#[cfg(target_arch = "wasm32")]
fn main() {}
