pub mod app;
pub mod core;
pub mod features;
pub mod platform;
pub mod runtime;
pub mod screens;
pub mod ui;

pub use core::model::{AppPhase, MIN_SPLASH_MS, State};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn main() {
    platform::init();
    runtime::run();
}
