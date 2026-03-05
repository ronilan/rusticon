use little_tui::*;
use little_tui_collection::{App, AppOptions};

use crate::{
    core::{
        io::RusticonIo,
        model::{AppPhase, ExitFlow, State, MIN_SPLASH_LOOPS, MIN_SPLASH_MS},
    },
    screens, ui,
};

fn terminal_too_small() -> bool {
    Terminal::columns() < ui::APP_WIDTH || Terminal::rows() < ui::APP_HEIGHT
}

fn build_app(io: impl RusticonIo + Clone + 'static) -> App<State> {
    let app = App::new(AppOptions {
        height: None,
        draw_on_window_resize: false,
        ..Default::default()
    });
    let splash_layer = screens::splash::splash::build();
    splash_layer.showed(false);
    let main_layer = screens::editor::screen::build();
    main_layer.showed(false);
    let message_layer = screens::message::screen::build();
    message_layer.showed(false);
    let title_bar = ui::title_bar::build();
    title_bar.showed(false);
    let viewport_guard_layer = ui::viewport_guard::build();
    viewport_guard_layer.showed(false);

    app.on_window(|el: &App<State>, state: &mut State, event: &EventWindow| {
        if event.window == Window::Resize {
            el.elements_to_center();
            state.viewport_too_small = terminal_too_small();
            el.draw();
        }
    });

    let io_for_loop = io.clone();

    app.add(splash_layer);
    app.add(main_layer);
    app.add(message_layer);
    app.add(title_bar);
    app.add(viewport_guard_layer);
    app.elements_to_center();

    app.on_loop(move |el, state, event| {
        if event.loop_count == 0 {
            el.draw();
        }

        let phase_before = state.phase.clone();

        if state.viewport_too_small {
            return;
        }

        if state.phase != AppPhase::Splash {
            if state.phase == AppPhase::Message {
                let now = Globals::now();
                let mut should_save = false;
                let mut should_exit = false;

                if let ExitFlow::SaveThenExit {
                    save_done,
                    started_ms,
                } = &mut state.exit_flow
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
                    exit();
                }
            }

            return;
        }

        state.splash_loop_count += 1;
        if state.splash_started_ms.is_none() {
            state.splash_started_ms = Some(Globals::now());
        }

        let splash_elapsed = state
            .splash_started_ms
            .map(|start| Globals::now() - start)
            .unwrap_or(0.0);

        if state.splash_loop_count < MIN_SPLASH_LOOPS || splash_elapsed < MIN_SPLASH_MS {
            return;
        }

        if let Some(import_result) = io_for_loop.take_import_result() {
            match import_result {
                Ok((data, palette, icon_size, returned_path)) => {
                    Globals::set_tick_rate(33.0);
                    state.phase = AppPhase::Main;
                    state.file_path = returned_path;
                    state.size = icon_size;
                    state.paintbrush = palette[0];
                    state.palette_index = 0;
                    state.palette_colors = palette;
                    state.picker_mode = false;
                    state.candidate = None;

                    if icon_size == 16 {
                        state.canvas16_data = data;
                        state.canvas8_data = vec![None; 8 * 8];
                    } else {
                        state.canvas8_data = data;
                        state.canvas16_data = vec![None; 16 * 16];
                    }
                }
                Err(err_msg) => {
                    state.phase = AppPhase::Message;
                    state.message_text = Some(err_msg);
                    state.message_color = 196;
                }
            }
        }

        if state.phase != phase_before {
            el.draw();
        }
    });

    app
}

pub fn app_flow(io: impl RusticonIo + Clone + 'static) -> RunHandle<State> {
    Globals::set_tick_rate(10.0);
    let file_path = io.initial_file_path();

    io.reset_import_result();
    io.load_file_in_background(file_path);

    let app = build_app(io.clone());
    let initial_state = State {
        phase: AppPhase::Splash,
        viewport_too_small: false,
        splash_loop_count: 0,
        splash_started_ms: None,
        message_text: None,
        message_color: 196,
        exit_flow: ExitFlow::None,
        candidate: None,
        paintbrush: None,
        palette_index: 0,
        palette_colors: vec![None; 8],
        picker_mode: false,
        canvas16_data: vec![None; 16 * 16],
        canvas8_data: vec![None; 8 * 8],
        size: 8,
        save_flag: false,
        file_path: "favicon.svg".to_string(),
    };

    app.run(initial_state)
}
