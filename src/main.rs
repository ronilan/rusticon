mod export;
mod import;
mod message;
mod native_io;
mod application_flow;
mod rusticon_screen;
mod shared;
mod splash_screen;
mod ui;

use native_io::NativeIo;

#[derive(Clone, Debug, PartialEq)]
pub struct SplashState {
    pub loop_count: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct State {
    pub app_x: isize,
    pub app_y: isize,
    pub candidate: Option<u8>,
    pub paintbrush: Option<u8>,
    pub palette_index: usize,
    pub palette_colors: Vec<Option<u8>>,
    pub picker_mode: bool,
    pub canvas16_data: Vec<Option<u8>>,
    pub canvas8_data: Vec<Option<u8>>,
    pub size: u8,
    pub save_flag: bool,
    pub file_path: String,
}

pub const MIN_SPLASH_LOOPS: usize = 20;

fn main() {
    let io = NativeIo::new();
    application_flow::run_flow(&io, |final_ui_state| io.handle_final_save(final_ui_state));
}
