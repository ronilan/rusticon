use crate::{
    core::model::AppPhase,
    ui::{APP_HEIGHT, APP_WIDTH},
    State,
};
use little_tui::*;
use little_tui_collection::Rectangle;

pub fn build() -> Rectangle<State> {
    let wrapper: Rectangle<State> = Rectangle::new();
    wrapper.showed(false);
    wrapper.width(APP_WIDTH).height(APP_HEIGHT).fill(Some(' '));
    wrapper.on_state(|el, state| {
        el.showed(!state.flow.viewport_too_small && state.flow.phase == AppPhase::Launch);
    });

    // Launch uses a static logo.
    wrapper.add(super::ui::launch_logo::build());
    wrapper.elements_snap_center_y();
    
    wrapper.add(crate::screens::splash::ui::splash_footer::build());
    wrapper.add(super::ui::launch_hint::build());
    wrapper.add(super::ui::start_new::build());
    wrapper.elements_snap_center_x();

    wrapper
}
