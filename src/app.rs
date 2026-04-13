use little_tui::*;
use little_tui_collection::{App, AppOptions};

#[path = "app/core/mod.rs"]
pub mod core;
#[path = "app/features/mod.rs"]
pub mod features;
#[path = "app/screens/mod.rs"]
pub mod screens;
#[path = "app/ui/mod.rs"]
pub mod ui;

use crate::platform;
use core::{
    io::RusticonIo,
    model::{AppPhase, ExitFlow, State, MIN_SPLASH_MS},
};
use ui::{APP_HEIGHT, APP_WIDTH};

pub fn build(io: impl RusticonIo + Clone + 'static) -> App<State> {
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

    let io_for_loop = io.clone();

    let app = App::new(AppOptions {
        height: None,
        draw_on_window_resize: false,
        draw_on_initialization: false,
        output_provider: platform::app_output_provider,
        ..Default::default()
    });

    app.on_window(|el: &App<State>, state: &mut State, event: &EventWindow| {
        if event.window == Window::Resize {
            el.elements_to_center();
            state.flow.viewport_too_small =
                Platform::columns() < APP_WIDTH || Platform::rows() < APP_HEIGHT;
            el.draw();
        }
    })
    .on_loop(move |el, state, _event| {
        let phase_before = state.flow.phase.clone();

        if state.flow.viewport_too_small {
            return;
        }

        if state.flow.exit_flow == ExitFlow::ExitRequested {
            if io_for_loop.return_to_launch_on_exit() {
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
            if !(state.flow.launch_start_new || io_for_loop.launch_drop_ready()) {
                return;
            }

            if !state.flow.launch_import_started {
                let from_drop = io_for_loop.launch_drop_ready();
                state.flow.launch_start_new = false;
                state.flow.launch_import_started = true;
                io_for_loop.start_import(state.editor.file_path.clone());

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

            if let Some(import_result) = io_for_loop.take_import_result() {
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
                let now = Globals::now();
                let mut should_save = false;
                let mut should_exit = false;

                if let ExitFlow::SaveThenExit {
                    save_done,
                    started_ms,
                } = &mut state.flow.exit_flow
                {
                    if !*save_done {
                        *save_done = true;
                        should_save = true;
                    }

                    if started_ms.is_none() {
                        *started_ms = Some(now);
                    }

                    if let Some(start) = *started_ms {
                        if now - start >= 2000.0 {
                            should_exit = true;
                        }
                    }
                }

                if should_save {
                    io_for_loop.handle_final_save(state);
                }
                if should_exit {
                    if io_for_loop.return_to_launch_on_exit() {
                        back_to_launch(state);
                        if state.flow.phase != phase_before {
                            el.draw();
                        }
                    } else {
                        exit();
                    }
                }
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

        if let Some(import_result) = io_for_loop.take_import_result() {
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
