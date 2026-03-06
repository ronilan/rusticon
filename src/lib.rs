#[cfg(target_arch = "wasm32")]
mod app;

#[cfg(target_arch = "wasm32")]
pub use app::{core, features, platform, rusticon_screen, screens, splash_screen, ui};

#[cfg(target_arch = "wasm32")]
pub use core::model::{SplashState, State, MIN_SPLASH_LOOPS};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn main() {
    console_error_panic_hook::set_once();

    let io = platform::wasm_io::WasmIo::new();
    core::application_flow::run_flow(io);
}
