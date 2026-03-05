#[cfg(not(target_arch = "wasm32"))]
mod core;
#[cfg(not(target_arch = "wasm32"))]
mod features;
#[cfg(not(target_arch = "wasm32"))]
mod platform;
#[cfg(not(target_arch = "wasm32"))]
mod screens;
#[cfg(not(target_arch = "wasm32"))]
mod ui;

#[cfg(not(target_arch = "wasm32"))]
pub use core::model::{AppPhase, State, MIN_SPLASH_LOOPS};

#[cfg(not(target_arch = "wasm32"))]
use platform::native_io::NativeIo;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let io = NativeIo::new();
    let run_handle = core::app::app_flow(io);
    let _ = run_handle.wait_final_state();
}

#[cfg(target_arch = "wasm32")]
fn main() {}
