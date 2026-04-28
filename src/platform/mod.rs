use std::cell::RefCell;
use std::sync::Arc;
use crate::core::io::RusticonIo;

#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(not(target_arch = "wasm32"))]
pub use native::{FileHandle};
#[cfg(target_arch = "wasm32")]
pub use wasm::{FileHandle, DroppedData, WasmIo};

thread_local! {
    static IO_PROVIDER: RefCell<Option<Arc<dyn RusticonIo>>> = RefCell::new(None);
}

pub fn set_io(io: Arc<dyn RusticonIo>) {
    IO_PROVIDER.with(|c| *c.borrow_mut() = Some(io));
}

pub fn get_io() -> Arc<dyn RusticonIo> {
    IO_PROVIDER.with(|c| c.borrow().as_ref().expect("IO not initialized").clone())
}

pub fn init() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();
    
    set_io(Arc::new(io()));
}

pub fn io() -> impl crate::core::io::RusticonIo + Clone + 'static {
    #[cfg(not(target_arch = "wasm32"))]
    {
        native::NativeIo::new()
    }

    #[cfg(target_arch = "wasm32")]
    {
        wasm::WasmIo::new()
    }
}

/// Helper to convert a path String into a platform FileHandle
#[cfg(not(target_arch = "wasm32"))]
pub fn to_file_handle(path: String) -> Option<FileHandle> {
    Some(path)
}
#[cfg(target_arch = "wasm32")]
pub fn to_file_handle(_path: String) -> Option<FileHandle> {
    None
}
