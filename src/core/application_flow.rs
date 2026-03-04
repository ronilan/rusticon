use little_tui::*;
use little_tui_collection::{App, AppOptions};
use std::{cell::Cell, rc::Rc};

use crate::{
    core::{
        io::RusticonIo,
        model::{AppPhase, ExitFlow, State, MIN_SPLASH_LOOPS, MIN_SPLASH_MS},
    },
    screens,
    ui,
};

fn terminal_too_small() -> bool {
    Terminal::columns() < ui::APP_WIDTH || Terminal::rows() < ui::APP_HEIGHT
}

fn apply_loaded_state(
    state: &mut State,
    data: Vec<Option<u8>>,
    palette: Vec<Option<u8>>,
    size: u8,
    returned_path: String,
) {
    Globals::set_tick_rate(33.0);
    state.phase = AppPhase::Main;
    state.file_path = returned_path;
    state.size = size;
    state.paintbrush = palette[0];
    state.palette_index = 0;
    state.palette_colors = palette;
    state.picker_mode = false;
    state.candidate = None;

    if size == 16 {
        state.canvas16_data = data;
        state.canvas8_data = vec![None; 8 * 8];
    } else {
        state.canvas8_data = data;
        state.canvas16_data = vec![None; 16 * 16];
    }
}

fn build_app(io: impl RusticonIo + Clone + 'static) -> App<State> {
    let app = App::new(AppOptions {
        height: None,
        draw_on_window_resize: false,
        ..Default::default()
    });

    let splash_layer = screens::splash::splash::build();
    let main_layer = screens::editor::screen::build();
    main_layer.showed(false);

    let message_layer = screens::message::screen::build();
    message_layer.showed(false);

    let title_bar = ui::title_bar::build();
    title_bar.fused(true).showed(false);

    let viewport_guard_layer = ui::viewport_guard::build();
    let guard_visible = Rc::new(Cell::new(false));

    app.on_window(|el: &App<State>, _state: &mut State, event: &EventWindow| {
        if event.window == Window::Resize {
            el.elements_to_center();
            el.draw();
        }
    });

    let io_for_loop = io.clone();
    let guard_visible_for_loop = guard_visible.clone();

    app.add(splash_layer);
    app.add(main_layer);
    app.add(message_layer);
    app.add(title_bar);
    app.add(viewport_guard_layer);
    app.elements_to_center();

    let layers = app.elements().iter();
    let splash_ref = layers
        .first()
        .cloned()
        .expect("missing splash layer in app root");
    let main_ref = layers
        .get(1)
        .cloned()
        .expect("missing editor layer in app root");
    let message_ref = layers
        .get(2)
        .cloned()
        .expect("missing message layer in app root");
    let title_ref = layers
        .get(3)
        .cloned()
        .expect("missing title layer in app root");
    let guard_ref = layers
        .get(4)
        .cloned()
        .expect("missing viewport guard layer in app root");

    let apply_layer_visibility = move |state: &State, too_small: bool| {
        guard_ref.showed(too_small);
        if too_small {
            splash_ref.showed(false);
            main_ref.showed(false);
            message_ref.showed(false);
            title_ref.showed(false);
            return;
        }

        splash_ref.showed(state.phase == AppPhase::Splash);
        main_ref.showed(state.phase == AppPhase::Main);
        message_ref.showed(state.phase == AppPhase::Message);
        title_ref.showed(state.phase == AppPhase::Main);
    };

    app.on_loop(move |el, state, _event| {
        let too_small = terminal_too_small();
        let visibility_changed = too_small != guard_visible_for_loop.get();
        guard_visible_for_loop.set(too_small);
        let phase_before = state.phase.clone();

        if too_small {
            apply_layer_visibility(state, true);
            if visibility_changed {
                el.draw();
            }
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

            apply_layer_visibility(state, false);
            if visibility_changed {
                el.draw();
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
                    apply_loaded_state(state, data, palette, icon_size, returned_path);
                }
                Err(err_msg) => {
                    state.phase = AppPhase::Message;
                    state.message_text = Some(err_msg);
                    state.message_color = 196;
                }
            }
        }

        apply_layer_visibility(state, false);

        if visibility_changed || state.phase != phase_before {
            el.draw();
        }
    });

    app
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

pub fn run_flow(io: impl RusticonIo + Clone + 'static) {
    Globals::set_tick_rate(10.0);
    let file_path = io.initial_file_path();

    io.reset_import_result();
    io.load_file_in_background(file_path);

    let app = build_app(io.clone());
    let initial_state = State {
        phase: AppPhase::Splash,
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

    let run_handle = app.run(initial_state);

    run_handle.on_exit(move |_final_state| {
        #[cfg(target_arch = "wasm32")]
        show_app_terminated();
    });

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = run_handle.wait_final_state();
    }
}
