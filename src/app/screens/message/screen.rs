use crate::{
    core::model::AppPhase,
    ui::{APP_HEIGHT, APP_WIDTH},
    State,
};
use little_tui::*;
use little_tui_elements::Rectangle;

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

    wrapper.add(super::ui::message::build());

    wrapper.showed(false);

    wrapper
}
