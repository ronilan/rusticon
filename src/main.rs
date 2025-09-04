mod elements;
mod event_loop;
mod export;
mod import;
mod message;
mod rusticon_screen;
mod splash_screen;
mod tui_engine;

use std::{
    env,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use export::export_svg;
use import::import_file;
use message::draw_message;
use tui_engine::{run, Elements};

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

const MIN_SPLASH_LOOPS: usize = 20;
const SPLASH_DELAY_MS: u64 = 100; // ~10 FPS

fn load_file_in_background(
    path: String,
) -> Arc<Mutex<Option<Result<(Vec<Option<u8>>, Vec<Option<u8>>, u8, String), String>>>> {
    let result_holder = Arc::new(Mutex::new(None));
    let result_holder_thread = result_holder.clone();

    // Spawn background thread to import file
    thread::spawn(move || {
        let result = std::panic::catch_unwind(|| import_file(&path))
            .map_err(|e| format!("Panic in import_file: {:?}", e))
            .and_then(|res| res);

        let mut guard = result_holder_thread.lock().unwrap();
        *guard = Some(result);
    });

    result_holder
}

fn exit_splash(
    state: &SplashState,
    result_holder: &Mutex<Option<Result<(Vec<Option<u8>>, Vec<Option<u8>>, u8, String), String>>>,
) -> bool {
    // lock the result holder to check if background thread is done
    let guard = result_holder.lock().unwrap();
    let thread_done = guard.is_some();
    let min_time_reached = state.loop_count >= MIN_SPLASH_LOOPS;

    // exit when background thread finished and minimum splash time reached
    thread_done && min_time_reached
}

fn exit_ui(state: &AppState) -> bool {
    // exit UI loop when exit_flag is true
    state.exit_flag
}

fn handle_final_save(final_ui_state: &AppState) {
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

    // Shared state for background result
    let result_holder = load_file_in_background(file_path.clone());

    // Splash screen state
    let splash_state = SplashState { loop_count: 0 };
    let splash_elements: Elements<'_, SplashState> = splash_screen::build();

    // Run splash until result_holder contains Some(...)
    run(
        splash_state,
        splash_elements,
        Some(Duration::from_millis(SPLASH_DELAY_MS)),
        // run accepts an optional exit condition: a closure that returns true when the UI should exit.
        // wrap the closure in Some because run expects an Option<&dyn Fn(&S) -> bool>.
        // the closure takes one argument, which is the current state.
        Some(&|state| exit_splash(state, &result_holder)),
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
            //let elements: Elements<'_, AppState> = uix::build_elements();
            let elements: Elements<'_, AppState> = rusticon_screen::build();
            let final_ui_state = run(ui_state, elements, None, Some(&exit_ui));

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
