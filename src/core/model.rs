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
    /// Color under the cursor before the last paint stroke (used to undo the
    /// second click of a double-click before flood fill).
    pub prev_color_on_canvas: Option<u8>,
    pub palette_index: usize,
    pub palette_colors: Vec<Option<u8>>,
    pub picker_mode: bool,
    pub canvas16_data: Vec<Option<u8>>,
    pub canvas8_data: Vec<Option<u8>>,
    pub size: u8,
    pub save_flag: bool,
    pub save_requested: bool,
    pub file_path: String,
    pub file_handle: Option<crate::platform::FileHandle>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct State {
    pub flow: FlowState,
    pub editor: EditorState,
}

impl Default for State {
    fn default() -> Self {
        State {
            flow: FlowState {
                phase: AppPhase::Splash,
                viewport_too_small: false,
                launch_start_new: false,
                launch_import_started: false,
                splash_started_ms: None,
                message_text: None,
                message_color: 196,
                exit_flow: ExitFlow::None,
            },
            editor: EditorState {
                candidate: None,
                paintbrush: None,
                prev_color_on_canvas: None,
                palette_index: 0,
                palette_colors: vec![None; 8],
                picker_mode: false,
                canvas16_data: vec![None; 16 * 16],
                canvas8_data: vec![None; 8 * 8],
                size: 8,
                save_flag: false,
                save_requested: false,
                file_path: String::new(),
                file_handle: None,
            },
        }
    }
}

pub const MIN_SPLASH_MS: f64 = 2000.0;
