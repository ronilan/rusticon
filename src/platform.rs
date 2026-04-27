#[cfg(not(target_arch = "wasm32"))]
#[path = "app/platform/native_io.rs"]
pub mod native_io;
#[cfg(target_arch = "wasm32")]
#[path = "app/platform/wasm_io.rs"]
pub mod wasm_io;

#[cfg(not(target_arch = "wasm32"))]
pub use native_io::{FileHandle, DroppedData};
#[cfg(target_arch = "wasm32")]
pub use wasm_io::{FileHandle, DroppedData};

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

/// Helper to convert a path String into a platform FileHandle
pub fn to_file_handle(path: String) -> Option<FileHandle> {
    #[cfg(not(target_arch = "wasm32"))]
    { Some(path) }
    #[cfg(target_arch = "wasm32")]
    { let _ = path; None }
}
