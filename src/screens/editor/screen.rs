use crate::{
    core::model::{AppPhase, State},
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

    wrapper
        .on_state(|el, state| {
            el.showed(!state.flow.viewport_too_small && state.flow.phase == AppPhase::Main);
        })
        .on_mouse(|el, state, event| {
            if event.mouse == Mouse::Move && el.status().hovered.get() {
                state.editor.picker_mode = false;
            }
        });

    wrapper.add(super::ui::canvas_8::build());
    wrapper.add(super::ui::canvas_16::build());

    wrapper.add(super::ui::color_picker_16::build());
    wrapper.add(super::ui::color_picker_216::build());
    wrapper.add(super::ui::color_picker_gray::build());
    wrapper.add(super::ui::color_picker_empty::build());
    wrapper.add(super::ui::color_picker_palette::build());

    wrapper.add(super::ui::label_color_selected::build());
    wrapper.add(super::ui::color_selected::build());
    wrapper.add(super::ui::color_candidate::build());
    wrapper.add(super::ui::label_color_candidate::build());

    wrapper.add(super::ui::label_new::build());
    wrapper.add(super::ui::button_8::build());
    wrapper.add(super::ui::button_16::build());

    wrapper.add(super::ui::label_end::build());
    wrapper.add(super::ui::button_save::build());
    wrapper.add(super::ui::button_exit::build());

    wrapper.showed(false);

    wrapper
}
