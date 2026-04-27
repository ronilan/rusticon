use little_tui::{DeferredValue, Globals};

use crate::{
    app,
    core::{
        io::RusticonIo,
        model::{AppPhase, EditorState, ExitFlow, FlowState, State},
    },
};

pub fn run(io: impl RusticonIo + Clone + 'static) -> DeferredValue<State> {
    let initial_phase = io.initial_phase();

    let file_path = io.initial_file_path();
    let initial_state = State {
        flow: FlowState {
            phase: initial_phase.clone(),
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
            palette_index: 0,
            palette_colors: vec![None; 8],
            picker_mode: false,
            canvas16_data: vec![None; 16 * 16],
            canvas8_data: vec![None; 8 * 8],
            size: 8,
            save_flag: false,
            file_path: file_path.clone(),
            file_handle: crate::platform::to_file_handle(file_path.clone()),
        },
    };

    Globals::set_tick_rate(10.0);
    if initial_phase != AppPhase::Launch {
        io.start_import(file_path);
    }

    let app = app::build(io.clone());
    app.run(initial_state)
}
