use little_tui::{run as tui_run, setup, Globals, Providers};

#[cfg(not(target_arch = "wasm32"))]
use little_tui_event_loop_terminal::run_event_loop as looper;
#[cfg(not(target_arch = "wasm32"))]
use little_tui_input_crossterm::CrosstermInput;
#[cfg(not(target_arch = "wasm32"))]
use little_tui_output_terminal::AnsiOutput;

#[cfg(target_arch = "wasm32")]
use little_tui_event_loop_browser::run_event_loop_wasm as looper;
#[cfg(target_arch = "wasm32")]
use little_tui_input_browser::BrowserInput;
#[cfg(target_arch = "wasm32")]
use little_tui_output_browser::BrowserOutput;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{prelude::Closure, JsCast};

use crate::{core::io::RusticonIo, rusticon_screen, splash_screen, SplashState, State};

#[cfg(not(target_arch = "wasm32"))]
fn setup_runtime<S: Clone + PartialEq + 'static>() {
    setup(Providers {
        input: Box::new(CrosstermInput::new()),
        output: Box::new(AnsiOutput::new(Box::new(CrosstermInput::new()))),
    });
}

#[cfg(target_arch = "wasm32")]
fn setup_runtime<S: Clone + PartialEq + 'static>() {
    setup(Providers {
        input: Box::new(BrowserInput::new()),
        output: Box::new(BrowserOutput::new()),
    });
}

fn build_initial_state(
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

#[cfg(target_arch = "wasm32")]
fn show_app_terminated() {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(body) = document.body() {
                body.set_inner_html(r#"<div id="terminal">App Terminated</div>"#);
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn run_flow(io: &impl RusticonIo) {
    let file_path = io.initial_file_path();

    io.reset_import_result();
    io.load_file_in_background(file_path);

    let splash_state = SplashState { started_ms: None };
    let splash_root = splash_screen::build();

    Globals::set_tick_rate(10.0);
    setup_runtime::<SplashState>();

    let splash_handle = tui_run(splash_root, splash_state, looper, |_| {});
    let _ = splash_handle.get();

    match io.take_import_result() {
        Some(Ok((data, palette, icon_size, returned_path))) => {
            let ui_state = build_initial_state(data, palette, icon_size, returned_path);
            let root = rusticon_screen::build();

            Globals::set_tick_rate(33.0);
            setup_runtime::<State>();

            let main_handle = tui_run(root, ui_state, looper, |_| {});
            let final_ui_state = main_handle.get();
            io.handle_final_save(&final_ui_state);
        }
        Some(Err(err_msg)) => io.finish_with_error(&err_msg, 196),
        None => io.finish_with_error("Background thread failed before sending result.", 196),
    }
}

#[cfg(target_arch = "wasm32")]
pub fn run_flow(io: impl RusticonIo + Clone + 'static) {
    let file_path = io.initial_file_path();

    io.reset_import_result();
    io.load_file_in_background(file_path);

    let splash_state = SplashState { started_ms: None };
    let splash_root = splash_screen::build();

    Globals::set_tick_rate(10.0);
    setup_runtime::<SplashState>();

    let splash_handle = tui_run(splash_root, splash_state, |_| {});
    let io_after_splash = io.clone();

    splash_handle.on_exit(
        move |_final_splash_state| match io_after_splash.take_import_result() {
            Some(Ok((data, palette, icon_size, returned_path))) => {
                let io_for_main = io_after_splash.clone();
                let callback = Closure::<dyn FnMut()>::new(move || {
                    let ui_state = build_initial_state(
                        data.clone(),
                        palette.clone(),
                        icon_size,
                        returned_path.clone(),
                    );
                    let root = rusticon_screen::build();

                    Globals::set_tick_rate(33.0);
                    setup_runtime::<State>();

                    let main_handle = tui_run(root, ui_state, |_| {});
                    let io_after_main = io_for_main.clone();
                    main_handle.on_exit(move |final_ui_state| {
                        io_after_main.handle_final_save(&final_ui_state);
                        show_app_terminated();
                    });
                });

                if let Some(window) = web_sys::window() {
                    let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                        callback.as_ref().unchecked_ref(),
                        0,
                    );
                }
                callback.forget();
            }
            Some(Err(err_msg)) => io_after_splash.finish_with_error(&err_msg, 196),
            None => io_after_splash.finish_with_error("Import bootstrap result missing.", 196),
        },
    );
}
