pub mod app;
pub mod platform;
pub mod runtime;
pub mod core;
pub mod features;
pub mod screens;
pub mod ui;

pub use core::model::{AppPhase, State, MIN_SPLASH_MS};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn main() {
    platform::init();
    runtime::run(platform::io());
}
