use crate::{
    core::model::AppPhase,
    ui::{APP_HEIGHT, APP_WIDTH},
    State,
};
use little_tui::*;
use little_tui_elements::Rectangle;

pub fn build() -> Rectangle<State> {
    let wrapper: Rectangle<State> = Rectangle::new();
    wrapper.width(APP_WIDTH).height(APP_HEIGHT).fill(Some(' '));
    wrapper.on_state(|el, state| {
        el.showed(!state.flow.viewport_too_small && state.flow.phase == AppPhase::Splash);
    });

    wrapper.add(super::ui::splash_logo::build());
    wrapper.elements_snap_center_y();

    wrapper.add(super::ui::splash_footer::build());
    wrapper.elements_snap_center_x();

    wrapper.showed(false);

    wrapper
}
