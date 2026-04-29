use little_tui::*;
use little_tui_elements::{App, AppOptions};

pub use crate::core::{
    io::RusticonIo,
    model::{AppPhase, ExitFlow, State, MIN_SPLASH_MS},
};

use crate::platform;
use crate::screens;
use crate::ui;

pub fn build() -> App<State> {
    fn back_to_launch(state: &mut State) {
        state.flow.phase = AppPhase::Launch;
        state.flow.launch_start_new = false;
        state.flow.launch_import_started = false;
        state.flow.splash_started_ms = None;
        state.flow.message_text = None;
        state.flow.message_color = 196;
        state.flow.exit_flow = ExitFlow::None;
        state.editor.save_flag = false;
        Globals::set_tick_rate(10.0);
    }

    let app = App::new(AppOptions {
        height: None,
        draw_on_window_resize: false,
        draw_on_initialization: false,
        ..Default::default()
    });

    app.on_window(|el: &App<State>, state: &mut State, event: &EventWindow| {
        if event.window == Window::Resize {
            el.elements_to_center();
            state.flow.viewport_too_small =
                Platform::columns() < ui::APP_WIDTH || Platform::rows() < ui::APP_HEIGHT;
            el.draw();
        }
    })
    .on_loop(move |el, state, _event| {
        platform::setup_macos_hooks();
        let io = platform::get_io();

        if io.launch_drop_ready()
            && state.flow.phase != AppPhase::Launch
        {
            state.flow.phase = AppPhase::Launch;
        }

        if state.editor.save_requested {
            state.editor.save_requested = false;
            io.perform_save(state);
        }

        if let Some(handle) = io.take_pending_handle() {
            state.editor.file_handle = Some(handle);
        }

        let phase_before = state.flow.phase.clone();

        if state.flow.viewport_too_small {
            return;
        }

        if state.flow.exit_flow == ExitFlow::ExitRequested {
            if io.return_to_launch_on_exit() {
                back_to_launch(state);
                if state.flow.phase != phase_before {
                    el.draw();
                }
            } else {
                exit();
            }
            return;
        }

        if state.flow.phase == AppPhase::Launch {
            if !(state.flow.launch_start_new || io.launch_drop_ready()) {
                return;
            }

            if !state.flow.launch_import_started {
                let from_drop = io.launch_drop_ready();
                state.flow.launch_start_new = false;
                state.flow.launch_import_started = true;
                io.start_import(state.editor.file_path.clone());

                if from_drop {
                    state.flow.phase = AppPhase::Splash;
                    state.flow.splash_started_ms = None;
                    Globals::set_tick_rate(10.0);
                    if state.flow.phase != phase_before {
                        el.draw();
                    }
                    return;
                }
            }

            if let Some(import_result) = io.take_import_result() {
                state.flow.launch_import_started = false;
                match import_result {
                    Ok((data, palette, icon_size, returned_path)) => {
                        Globals::set_tick_rate(33.0);
                        state.flow.phase = AppPhase::Main;
                        state.editor.file_path = returned_path;
                        state.editor.size = icon_size;
                        state.editor.paintbrush = palette[0];
                        state.editor.palette_index = 0;
                        state.editor.palette_colors = palette;
                        state.editor.picker_mode = false;
                        state.editor.candidate = None;

                        if icon_size == 16 {
                            state.editor.canvas16_data = data;
                            state.editor.canvas8_data = vec![None; 8 * 8];
                        } else {
                            state.editor.canvas8_data = data;
                            state.editor.canvas16_data = vec![None; 16 * 16];
                        }
                    }
                    Err(err_msg) => {
                        state.flow.phase = AppPhase::Message;
                        state.flow.message_text = Some(err_msg);
                        state.flow.message_color = 196;
                    }
                }
            }

            if state.flow.phase != phase_before {
                el.draw();
            }
            return;
        }

        if state.flow.phase != AppPhase::Splash {
            if state.flow.phase == AppPhase::Message {
                // messages can be cleared by clicking or after timeout, 
                // but for now we just let them stay or be handled by specific logic.
                // Minimal change: just return for now.
            }

            return;
        }

        if state.flow.splash_started_ms.is_none() {
            state.flow.splash_started_ms = Some(Globals::now());
        }

        let splash_elapsed = state
            .flow
            .splash_started_ms
            .map(|start| Globals::now() - start)
            .unwrap_or(0.0);

        if splash_elapsed < MIN_SPLASH_MS {
            return;
        }

        if let Some(import_result) = io.take_import_result() {
            state.flow.launch_import_started = false;
            match import_result {
                Ok((data, palette, icon_size, returned_path)) => {
                    Globals::set_tick_rate(33.0);
                    state.flow.phase = AppPhase::Main;
                    state.editor.file_path = returned_path;
                    state.editor.size = icon_size;
                    state.editor.paintbrush = palette[0];
                    state.editor.palette_index = 0;
                    state.editor.palette_colors = palette;
                    state.editor.picker_mode = false;
                    state.editor.candidate = None;

                    if icon_size == 16 {
                        state.editor.canvas16_data = data;
                        state.editor.canvas8_data = vec![None; 8 * 8];
                    } else {
                        state.editor.canvas8_data = data;
                        state.editor.canvas16_data = vec![None; 16 * 16];
                    }
                }
                Err(err_msg) => {
                    state.flow.phase = AppPhase::Message;
                    state.flow.message_text = Some(err_msg);
                    state.flow.message_color = 196;
                }
            }
        }

        if state.flow.phase != phase_before {
            el.draw();
        }
    });

    app.add(screens::launch::screen::build());
    app.add(screens::splash::screen::build());
    app.add(screens::editor::screen::build());
    app.add(screens::message::screen::build());
    app.add(ui::title_bar::build());
    app.add(ui::viewport_guard::build());

    app
}
