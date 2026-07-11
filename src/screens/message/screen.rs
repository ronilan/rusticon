use crate::{
    core::model::{AppPhase, State},
    platform,
    ui::{APP_HEIGHT, APP_WIDTH},
};
use incredible::*;
use incredible_elements::Rectangle;

pub fn build() -> Rectangle<State> {
    let wrapper: Rectangle<State> = Rectangle::new();
    wrapper
        .x(0)
        .y(1)
        .width(APP_WIDTH)
        .height(APP_HEIGHT.saturating_sub(1))
        .fill(Some(' '));
    wrapper.on_state(|el, state| {
        el.showed(!state.flow.viewport_too_small && state.flow.phase == AppPhase::Message);
    });

    wrapper.on_mouse(|_el, state, event| {
        if event.mouse == Mouse::Click && state.flow.phase == AppPhase::Message {
            let io = platform::get_io();
            let initial_phase = io.initial_phase();

            // Clear the broken path and reset to default
            state.editor.file_path = "favicon.svg".to_string();

            if initial_phase == AppPhase::Splash {
                state.flow.splash_started_ms = None;
                io.start_import(state.editor.file_path.clone());
            }

            state.flow.phase = initial_phase;
        }
    });

    wrapper.add(super::ui::message::build());

    wrapper.showed(false);

    wrapper
}
