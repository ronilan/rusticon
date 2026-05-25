use crate::core::model::State;
use incredible::*;
use incredible_elements::{Rectangle, Text};
use incredible_helpers_layout::arrangers::Arrangers;

pub fn build() -> Rectangle<State> {
    let guard: Rectangle<State> = Rectangle::new();
    guard.x(0).y(0).fused(true).showed(false).fill(Some(' '));
    guard.width(Platform::columns()).height(Platform::rows());
    guard.on_state(|el, state| {
        let should_show = state.flow.viewport_too_small;
        if el.get_showed() != should_show {
            el.showed(should_show);
            el.draw();
        }
    });

    let text: Text<State> = Text::default();
    text.text("Enlarge Terminal Window");
    guard.add(text);
    guard.elements_to_center();

    guard.on_window(|el: &Rectangle<State>, _state, event| {
        if event.window == Window::Resize {
            el.width(event.columns).height(event.rows);
            el.elements_to_center();
            if el.get_showed() {
                el.draw();
            }
        }
    });

    guard
}
