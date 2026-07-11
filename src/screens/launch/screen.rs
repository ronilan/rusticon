use crate::{
    core::model::{AppPhase, State},
    ui::{APP_HEIGHT, APP_WIDTH},
};
use incredible::*;
use incredible_elements::Rectangle;
use incredible_helpers_layout::{Arrangers, Flowers};

pub fn build() -> Rectangle<State> {
    let wrapper: Rectangle<State> = Rectangle::new();
    wrapper.width(APP_WIDTH).height(APP_HEIGHT).fill(Some(' '));
    wrapper.on_state(|el, state| {
        el.showed(!state.flow.viewport_too_small && state.flow.phase == AppPhase::Launch);
    });

    // Launch uses a static logo.
    let logo = crate::ui::logo::build();
    wrapper.add(logo);

    // Launch uses a static bouncer.
    let tagline = crate::ui::tagline::build();
    tagline.animation(None);
    wrapper.add(tagline);
    wrapper.elements_flow_down(0);

    wrapper.elements_to_center();

    wrapper.add(crate::ui::made_with::build());
    wrapper.add(super::ui::launch_hint::build());
    wrapper.add(super::ui::start_new::build());
    wrapper.elements_snap_center_x();

    wrapper.showed(false);

    wrapper
}
