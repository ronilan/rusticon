#[cfg(not(target_arch = "wasm32"))]
pub mod native_io;
#[cfg(target_arch = "wasm32")]
pub mod wasm_io;
