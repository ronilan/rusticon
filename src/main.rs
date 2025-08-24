mod event_loop;
mod export;
mod import;
mod message;
mod splash;
mod tui_engine;
mod ui;

use std::{
    env,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use export::export_svg;
use import::import_file;
use message::draw_message;
use tui_engine::{run_tui, Elements};

#[derive(Clone, Debug, PartialEq)]
pub struct SplashState {
    pub loop_count: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AppState {
    app_x: u16,
    app_y: u16,
    candidate: Option<u8>,
    paintbrush: Option<u8>,
    palette_index: usize,
    palette_colors: Vec<Option<u8>>,
    picker_mode: bool,
    canvas16_data: Vec<Option<u8>>,
    canvas8_data: Vec<Option<u8>>,
    size: u8,
    save_flag: bool,
    exit_flag: bool,
    file_path: String,
}

fn exit_ui(state: &AppState) -> bool {
    if state.save_flag {
        let (data, size) = if state.size == 16 {
            (state.canvas16_data.clone(), 16)
        } else {
            (state.canvas8_data.clone(), 8)
        };

        match export_svg(
            &data,
            &state.palette_colors,
            size,
            size,
            10,
            &state.file_path,
        ) {
            Ok(_) => draw_message("Export succeeded!", 10),
            Err(err_msg) => draw_message(&err_msg, 196),
        }
    }

    state.exit_flag
}

fn main() {
    // Default empty canvases
    let mut canvas16_data = vec![None; 16 * 16];
    let mut canvas8_data = vec![None; 8 * 8];
    let mut file_path = "favicon.svg".to_string();

    // Command-line argument
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        file_path = args[1].clone();
    }

    // Shared state for background result
    let result_holder: Arc<
        Mutex<Option<Result<(Vec<Option<u8>>, Vec<Option<u8>>, u8, String), String>>>,
    > = Arc::new(Mutex::new(None));
    let result_holder_thread = result_holder.clone();
    let file_path_clone = file_path.clone();

    // Spawn background thread to import file
    thread::spawn(move || {
        let result = std::panic::catch_unwind(|| import_file(&file_path_clone))
            .map_err(|_| "Panic in import_file".to_string())
            .and_then(|res| res);

        let mut guard = result_holder_thread.lock().unwrap();
        *guard = Some(result);
    });

    // Splash screen state
    let splash_state = SplashState { loop_count: 0 };
    let splash_elements: Elements<'_, SplashState> = splash::build_elements();

    // Run splash until result_holder contains Some(...)
    let result_holder_main = result_holder.clone();
    run_tui(
        splash_elements,
        splash_state,
        Some(Duration::from_millis(66)),
        // run_tui accepts an optional exit condition: a closure that returns true when the UI should exit.
        // wrap the closure in Some because run_tui expects an Option<&dyn Fn(&S) -> bool>.
        // the closure takes one argument, which is usually the current state (&S) — ignore it here.
        Some(&|state: &SplashState| {
            let guard = result_holder_main.lock().unwrap();
            let thread_done = guard.is_some();
            let min_time_reached = state.loop_count >= 20;

            // Exit only if both the thread is done AND minimum loops have passed
            thread_done && min_time_reached
        }),
    );

    // Retrieve the final result
    let final_result = result_holder.lock().unwrap().take();
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
            let ui_state = AppState {
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
                exit_flag: false,
                file_path: returned_path,
            };

            // Run main UI
            let elements: Elements<'_, AppState> = ui::build_elements();
            let final_ui_state = run_tui(elements, ui_state, None, Some(&exit_ui));

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
                    30,
                    &final_ui_state.file_path,
                ) {
                    Ok(_) => draw_message("Export succeeded!", 10),
                    Err(err_msg) => draw_message(&err_msg, 196),
                }
            }
        }
        Some(Err(err_msg)) => {
            draw_message(&err_msg, 196);
            std::process::exit(1);
        }
        None => {
            draw_message("Background thread failed before sending result.", 196);
            std::process::exit(1);
        }
    }
}
