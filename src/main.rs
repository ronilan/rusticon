mod export;
mod import;
mod message;
mod rusticon_screen;
mod shared;
mod splash_screen;
mod ui;

use std::{env, thread};

use little_tui::{run, setup, Globals, Providers};
use little_tui_event_loop::run_event_loop;
use little_tui_input_crossterm::CrosstermInput;
use little_tui_output_terminal::AnsiOutput;

use export::export_svg;
use import::import_file;
use message::draw_message;

#[derive(Clone, Debug, PartialEq)]
pub struct SplashState {
    pub loop_count: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct State {
    app_x: isize,
    app_y: isize,
    candidate: Option<u8>,
    paintbrush: Option<u8>,
    palette_index: usize,
    palette_colors: Vec<Option<u8>>,
    picker_mode: bool,
    canvas16_data: Vec<Option<u8>>,
    canvas8_data: Vec<Option<u8>>,
    size: u8,
    save_flag: bool,
    file_path: String,
}

const MIN_SPLASH_LOOPS: usize = 20;

fn load_file_in_background(path: String) {
    // Use the global shared holder
    let result_holder_thread = crate::shared::RESULT_HOLDER.clone();

    thread::spawn(move || {
        let result = std::panic::catch_unwind(|| import_file(&path))
            .map_err(|e| format!("Panic in import_file: {:?}", e))
            .and_then(|res| res);

        let mut guard = result_holder_thread.lock().unwrap();
        *guard = Some(result);
    });
}

fn handle_final_save(final_ui_state: &State) {
    // Final save on exit if requested
    if final_ui_state.save_flag {
        let (data, size) = if final_ui_state.size == 16 {
            (final_ui_state.canvas16_data.clone(), 16)
        } else {
            (final_ui_state.canvas8_data.clone(), 8)
        };

        match export_svg(
            &data,
            &final_ui_state.palette_colors,
            size,
            size,
            32,
            &final_ui_state.file_path,
        ) {
            Ok(_) => draw_message("Export succeeded!", 10),
            Err(err_msg) => draw_message(&err_msg, 196),
        }
    }
}

fn main() {
    // Default empty canvases
    let mut canvas16_data = vec![None; 16 * 16];
    let mut canvas8_data = vec![None; 8 * 8];

    // Command-line argument
    let file_path = env::args()
        .nth(1)
        .unwrap_or_else(|| "favicon.svg".to_string());

    // Clear global result holder and spawn background loader
    {
        let mut guard = shared::RESULT_HOLDER.lock().unwrap();
        *guard = None;
    }
    load_file_in_background(file_path.clone());

    // Splash screen state
    let splash_state = SplashState { loop_count: 0 };
    let splash_root = splash_screen::build();

    // Run splash until result_holder contains Some(...)
    Globals::set_tick_rate(10.0);
    setup(Providers {
        input: Box::new(CrosstermInput::new()),
        output: Box::new(AnsiOutput::new(Box::new(CrosstermInput::new()))),
        looper: run_event_loop::<SplashState>,
    });
    run(splash_root, splash_state);

    // Retrieve the final result
    let final_result = shared::RESULT_HOLDER.lock().unwrap().take();
    match final_result {
        Some(Ok((data, palette, icon_size, returned_path))) => {
            let paintbrush = palette[0];
            let size = icon_size;
            if size == 16 {
                canvas16_data = data;
            } else {
                canvas8_data = data;
            }

            // Build initial app state
            let ui_state = State {
                app_x: 0,
                app_y: 0,
                candidate: None,
                paintbrush,
                palette_index: 0,
                palette_colors: palette,
                picker_mode: false,
                canvas16_data,
                canvas8_data,
                size,
                save_flag: false,
                file_path: returned_path,
            };
            let root = rusticon_screen::build();

            // Run main UI
            Globals::set_tick_rate(33.0);
            setup(Providers {
                input: Box::new(CrosstermInput::new()),
                output: Box::new(AnsiOutput::new(Box::new(CrosstermInput::new()))),
                looper: run_event_loop::<State>,
            });
            let final_ui_state = run(root, ui_state);

            // Final save if needed
            handle_final_save(&final_ui_state);
        }
        Some(Err(err_msg)) => {
            // Display error message if import failed
            draw_message(&err_msg, 196);
            std::process::exit(1);
        }
        None => {
            // Background thread failed without sending a result
            draw_message("Background thread failed before sending result.", 196);
            std::process::exit(1);
        }
    }
}
