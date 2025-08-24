mod event_loop;
mod export;
mod import;
mod message;
mod splash;
mod tui_engine;
mod ui;

use std::env;
use std::time::Duration;

use export::export_svg;
use import::crumbicon;
use message::draw_message;
use tui_engine::{run_tui, Elements};

#[derive(Clone, Debug, PartialEq)]
pub struct SplashState {
    pub exit_flag: bool,
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
    mouse_x: u16,
    mouse_y: u16,
    canvas16_data: Vec<Option<u8>>,
    canvas8_data: Vec<Option<u8>>,
    size: u8,
    save_flag: bool,
    exit_flag: bool,
    file_path: String,
}

fn exit_splash(state: &SplashState) -> bool {
    state.exit_flag
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
    // Default empty canvas
    let mut palette_colors = vec![None; 8];
    let mut canvas16_data = vec![None; 16 * 16];
    let mut canvas8_data = vec![None; 8 * 8];
    let mut size = 8;
    let mut file_path = "favicon.svg".to_string();
    let mut paintbrush = None;

    // Command-line argument
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        file_path = args[1].clone();
    }

    // Try to load crumbicon if file exists
    if std::path::Path::new(&file_path).exists() {
        match crumbicon(&file_path) {
            Ok((data, palette, icon_size)) => {
                palette_colors = palette;
                paintbrush = palette_colors[0];
                size = icon_size;
                if size == 16 {
                    canvas16_data = data;
                } else {
                    canvas8_data = data;
                }
            }
            Err(err_msg) => {
                draw_message(&err_msg, 196); // File exists but invalid
                std::process::exit(1); // exit on invalid file
            }
        }
    } // else  File missing → silent blank canvas

    // Splash screen
    let splash_state = SplashState { exit_flag: false };
    let splash_elements: Elements<'_, SplashState> = splash::build_elements();
    run_tui(
        splash_elements,
        splash_state,
        Some(Duration::from_millis(66)),
        Some(&exit_splash),
    );

    // Main UI
    let ui_state = AppState {
        app_x: 0,
        app_y: 0,
        candidate: None,
        paintbrush,
        palette_index: 0,
        palette_colors,
        picker_mode: false,
        mouse_x: 0,
        mouse_y: 0,
        canvas16_data,
        canvas8_data,
        size,
        save_flag: false,
        exit_flag: false,
        file_path,
    };

    let elements: Elements<'_, AppState> = ui::build_elements();
    let final_ui_state = run_tui(elements, ui_state, None, Some(&exit_ui));

    // Final save after exiting UI
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
