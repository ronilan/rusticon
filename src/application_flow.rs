use little_tui::{run as tui_run, setup, Globals, Providers};
use little_tui_event_loop::run_event_loop;
use little_tui_input_crossterm::CrosstermInput;
use little_tui_output_terminal::AnsiOutput;

use crate::{native_io::RusticonIo, rusticon_screen, splash_screen, SplashState, State};

fn setup_terminal<S: Clone + PartialEq + 'static>() {
    setup(Providers {
        input: Box::new(CrosstermInput::new()),
        output: Box::new(AnsiOutput::new(Box::new(CrosstermInput::new()))),
        looper: run_event_loop::<S>,
    });
}

pub fn run_splash_phase(
    io: &impl RusticonIo,
    file_path: String,
) -> Option<crate::shared::ImportOutcome> {
    io.reset_import_result();
    io.load_file_in_background(file_path);

    let splash_state = SplashState { loop_count: 0 };
    let splash_root = splash_screen::build();

    Globals::set_tick_rate(10.0);
    setup_terminal::<SplashState>();
    let splash_handle = tui_run(splash_root, splash_state, |_| {});
    let _ = splash_handle.wait_final_state();

    io.take_import_result()
}

pub fn build_initial_state(
    data: Vec<Option<u8>>,
    palette: Vec<Option<u8>>,
    size: u8,
    returned_path: String,
) -> State {
    let mut canvas16_data = vec![None; 16 * 16];
    let mut canvas8_data = vec![None; 8 * 8];

    if size == 16 {
        canvas16_data = data;
    } else {
        canvas8_data = data;
    }

    State {
        app_x: 0,
        app_y: 0,
        candidate: None,
        paintbrush: palette[0],
        palette_index: 0,
        palette_colors: palette,
        picker_mode: false,
        canvas16_data,
        canvas8_data,
        size,
        save_flag: false,
        file_path: returned_path,
    }
}

pub fn run_main_phase(ui_state: State) -> State {
    let root = rusticon_screen::build();

    Globals::set_tick_rate(33.0);
    setup_terminal::<State>();

    let main_handle = tui_run(root, ui_state, |_| {});
    main_handle.wait_final_state()
}

pub fn run_flow(io: &impl RusticonIo, on_final_state: impl FnOnce(&State)) {
    let file_path = io.initial_file_path();

    let final_result = run_splash_phase(io, file_path);
    match final_result {
        Some(Ok((data, palette, icon_size, returned_path))) => {
            let ui_state = build_initial_state(data, palette, icon_size, returned_path);
            let final_ui_state = run_main_phase(ui_state);
            on_final_state(&final_ui_state);
        }
        Some(Err(err_msg)) => {
            io.report_message(&err_msg, 196);
            std::process::exit(1);
        }
        None => {
            io.report_message("Background thread failed before sending result.", 196);
            std::process::exit(1);
        }
    }
}
