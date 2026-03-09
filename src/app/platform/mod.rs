use little_tui::terminal::{TerminalInput, TerminalOutput};

#[cfg(not(target_arch = "wasm32"))]
pub mod native_io;
#[cfg(target_arch = "wasm32")]
pub mod wasm_io;

#[cfg(target_arch = "wasm32")]
use little_tui_output_browser::BrowserOutput;
#[cfg(not(target_arch = "wasm32"))]
use little_tui_output_terminal::AnsiOutput;

pub fn app_output_provider(input: Box<dyn TerminalInput>) -> Box<dyn TerminalOutput> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        Box::new(AnsiOutput::new(input))
    }

    #[cfg(target_arch = "wasm32")]
    {
        let _ = input;
        Box::new(BrowserOutput::new())
    }
}
