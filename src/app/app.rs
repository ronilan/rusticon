use little_tui::{Globals, RunHandle};

use super::{
    core::{
        io::RusticonIo,
        model::{AppPhase, ExitFlow, State},
    },
    ui,
};

pub fn app(io: impl RusticonIo + Clone + 'static) -> RunHandle<State> {
    Globals::set_tick_rate(10.0);
    let file_path = io.initial_file_path();

    io.reset_import_result();
    io.load_file_in_background(file_path);

    let app = ui::app::build(io.clone());

    let initial_state = State {
        phase: AppPhase::Splash,
        viewport_too_small: false,
        splash_loop_count: 0,
        splash_started_ms: None,
        message_text: None,
        message_color: 196,
        exit_flow: ExitFlow::None,
        candidate: None,
        paintbrush: None,
        palette_index: 0,
        palette_colors: vec![None; 8],
        picker_mode: false,
        canvas16_data: vec![None; 16 * 16],
        canvas8_data: vec![None; 8 * 8],
        size: 8,
        save_flag: false,
        file_path: "favicon.svg".to_string(),
    };

    app.run(initial_state)
}
