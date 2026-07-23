use crate::core::io::RusticonIo;
use std::cell::RefCell;
use std::sync::Arc;

#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(not(target_arch = "wasm32"))]
pub use native::FileHandle;
#[cfg(target_arch = "wasm32")]
pub use wasm::{DroppedData, FileHandle, WasmIo};

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

pub fn setup_macos_hooks() {
    #[cfg(all(not(target_arch = "wasm32"), target_os = "macos"))]
    native::setup_macos_hooks();
}

pub fn setup_windows_drop() {
    #[cfg(all(
        not(target_arch = "wasm32"),
        target_os = "windows",
        feature = "windows-native"
    ))]
    native::setup_windows_drop();
}

/// Handles paste events by checking if the pasted text is a valid file path.
/// If it is, stores the path in DROP_HOLDER for the import flow.
pub fn handle_paste(text: &str) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use crate::core::shared::DROP_HOLDER;
        // Trim whitespace/newlines that terminals may add when pasting file paths
        let mut path = text.trim();

        // Handle quoted paths (e.g., "/path/to/my file.png")
        if path.starts_with('"') && path.ends_with('"') && path.len() >= 2 {
            path = &path[1..path.len() - 1];
        }

        // Handle escaped spaces (e.g., /path/to/my\ file.png)
        // Terminals often escape spaces when pasting file paths
        let unescaped = path.replace("\\ ", " ");
        let path_ref: &str = &unescaped;

        if std::path::Path::new(path_ref).exists() {
            if let Ok(mut guard) = DROP_HOLDER.lock() {
                *guard = Some(unescaped);
            }
        }
    }

    // WASM: paste events are not used for file opening (drag-and-drop via DOM is used instead)
    #[cfg(target_arch = "wasm32")]
    {
        let _ = text;
    }
}
