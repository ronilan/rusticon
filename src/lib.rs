#[cfg(target_arch = "wasm32")]
mod app;

#[cfg(target_arch = "wasm32")]
pub use app::{core, features, platform, screens, ui};

#[cfg(target_arch = "wasm32")]
pub use core::model::{AppPhase, State, MIN_SPLASH_MS};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn main() {
    console_error_panic_hook::set_once();

    let io = platform::wasm_io::WasmIo::new();
    app::app(io);
}
