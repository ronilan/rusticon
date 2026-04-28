mod app;
mod platform;
mod runtime;

pub mod core;
pub mod features;
pub mod screens;
pub mod ui;

pub use crate::core::model::{AppPhase, State, MIN_SPLASH_MS};

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    platform::init();
    runtime::run(platform::io());
}

#[cfg(target_arch = "wasm32")]
fn main() {}
