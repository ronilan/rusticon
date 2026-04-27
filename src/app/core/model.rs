#[derive(Clone, Debug, PartialEq)]
pub enum AppPhase {
    Launch,
    Splash,
    Main,
    Message,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExitFlow {
    None,
    ExitRequested,
    SaveThenExit {
        save_done: bool,
        started_ms: Option<f64>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct FlowState {
    pub phase: AppPhase,
    pub viewport_too_small: bool,
    pub launch_start_new: bool,
    pub launch_import_started: bool,
    pub splash_started_ms: Option<f64>,
    pub message_text: Option<String>,
    pub message_color: u8,
    pub exit_flow: ExitFlow,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EditorState {
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
    pub file_handle: Option<crate::platform::FileHandle>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct State {
    pub flow: FlowState,
    pub editor: EditorState,
}

pub const MIN_SPLASH_MS: f64 = 2000.0;
