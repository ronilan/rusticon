#[derive(Clone, Debug, PartialEq)]
pub enum AppPhase {
    Splash,
    Main,
    Message,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExitFlow {
    None,
    SaveThenExit {
        save_done: bool,
        started_ms: Option<f64>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct State {
    pub phase: AppPhase,
    pub viewport_too_small: bool,
    pub splash_loop_count: usize,
    pub splash_started_ms: Option<f64>,
    pub message_text: Option<String>,
    pub message_color: u8,
    pub exit_flow: ExitFlow,
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
pub const MIN_SPLASH_MS: f64 = 2000.0;
