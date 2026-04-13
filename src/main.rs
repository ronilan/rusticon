mod app;
mod platform;
mod runtime;

pub use app::{core, features, screens, ui};
pub use core::model::{AppPhase, State, MIN_SPLASH_MS};

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    platform::init();
    runtime::run(platform::io());
}

#[cfg(target_arch = "wasm32")]
fn main() {}
