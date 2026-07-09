#[derive(Clone, Debug, PartialEq)]
pub struct SplashState {
    pub started_ms: Option<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct State {
    pub app_x: isize,
    pub app_y: isize,
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
    pub file_path: String,
}

pub const MIN_SPLASH_MS: f64 = 2000.0;
