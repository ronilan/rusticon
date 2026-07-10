#[cfg(not(target_arch = "wasm32"))]
pub mod native_io;
#[cfg(target_arch = "wasm32")]
pub mod wasm_io;

/// Platform-specific handle to an opened file.
///
/// On WASM this is the `FileSystemFileHandle` obtained from a drag-and-drop
/// (so we can persist back to the exact file the user opened). On native there
/// is no such handle — saving is driven by the file path instead.
#[cfg(target_arch = "wasm32")]
pub type FileHandle = wasm_bindgen::JsValue;
#[cfg(not(target_arch = "wasm32"))]
pub type FileHandle = ();
