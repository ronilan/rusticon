use crate::{
    app,
    core::model::{AppPhase, State},
    platform,
};
use incredible::{DeferredValue, Globals};

pub fn run() -> DeferredValue<State> {
    let io = platform::get_io();
    let initial_phase = io.initial_phase();
    let file_path = io.initial_file_path();

    let mut initial_state = State::default();
    initial_state.flow.phase = initial_phase.clone();
    initial_state.editor.file_path = file_path.clone();

    Globals::set_tick_rate(10.0);
    if initial_phase != AppPhase::Launch {
        io.start_import(file_path);
    }

    let app = app::build();
    app.run(initial_state)
}
