#[cfg(not(target_arch = "wasm32"))]
#[path = "app/platform/native_io.rs"]
pub mod native_io;
#[cfg(target_arch = "wasm32")]
#[path = "app/platform/wasm_io.rs"]
pub mod wasm_io;

pub fn init() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
}

pub fn io() -> impl crate::core::io::RusticonIo + Clone + 'static {
    #[cfg(not(target_arch = "wasm32"))]
    {
        native_io::NativeIo::new()
    }

    #[cfg(target_arch = "wasm32")]
    {
        wasm_io::WasmIo::new()
    }
}

