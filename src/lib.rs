#[cfg(target_arch = "wasm32")]
mod core;
#[cfg(target_arch = "wasm32")]
mod features;
#[cfg(target_arch = "wasm32")]
mod platform;
#[cfg(target_arch = "wasm32")]
mod screens;
#[cfg(target_arch = "wasm32")]
mod ui;

#[cfg(target_arch = "wasm32")]
pub use core::model::{AppPhase, State, MIN_SPLASH_LOOPS};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn main() {
    console_error_panic_hook::set_once();

    let io = platform::wasm_io::WasmIo::new();
    core::app::app_flow(io);
}
