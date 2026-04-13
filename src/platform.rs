use little_tui::*;

#[cfg(not(target_arch = "wasm32"))]
#[path = "app/platform/native_io.rs"]
pub mod native_io;
#[cfg(target_arch = "wasm32")]
#[path = "app/platform/wasm_io.rs"]
pub mod wasm_io;

#[cfg(target_arch = "wasm32")]
use little_tui_output_html::HtmlOutput;
#[cfg(not(target_arch = "wasm32"))]
use little_tui_output_terminal::AnsiOutput;

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

pub fn app_output_provider(input: Box<dyn PlatformInput>) -> Box<dyn PlatformOutput> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        Box::new(AnsiOutput::new(input))
    }

    #[cfg(target_arch = "wasm32")]
    {
        let _ = input;
        Box::new(HtmlOutput::new())
    }
}
