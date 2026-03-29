#[cfg(target_arch = "wasm32")]
mod app;
#[cfg(target_arch = "wasm32")]
mod platform;
#[cfg(target_arch = "wasm32")]
mod runtime;

#[cfg(target_arch = "wasm32")]
pub use app::{core, features, screens, ui};

#[cfg(target_arch = "wasm32")]
pub use core::model::{AppPhase, State, MIN_SPLASH_MS};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn main() {
    platform::init();
    runtime::run(platform::io());
}
