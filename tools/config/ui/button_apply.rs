use incredible::*;
use incredible_elements::Button;

use crate::state::State;

pub fn build_button_apply() -> Button<State> {
    let button_apply: Button<State> = Button::new();
    button_apply
        .text(" Apply ")
        .x(68)
        .y(16)
        .width(10)
        .focused(false);

    button_apply
        .on_key(|_el, state, event| {
            if event.key == Key::Enter && state.is_valid {
                state.should_apply = true;
            }
        })
        .on_mouse(|_el, state, event| {
            if event.mouse == Mouse::Click && state.is_valid {
                state.should_apply = true;
            }
        })
        .on_change(|el, state, _event| {
            if el.status().focused.get() {
                state.focused_index = 6;
            }
        })
        .on_state(|el, state| {
            el.disabled(!state.is_valid);
        });
    button_apply
}
