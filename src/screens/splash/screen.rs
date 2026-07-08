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
        el.showed(!state.flow.viewport_too_small && state.flow.phase == AppPhase::Splash);
    });

    // Splash uses animated logo
    let logo = crate::ui::logo::build();
    logo.animation(Some(Animation::new(2000.0, 8.0, 10.0)));
    wrapper.add(logo);

    wrapper.add(crate::ui::tagline::build());
    wrapper.elements_flow_down(0);
    wrapper.elements_to_center();

    wrapper.add(crate::ui::made_with::build());
    wrapper.elements_snap_center_x();

    wrapper.showed(false);

    wrapper
}
