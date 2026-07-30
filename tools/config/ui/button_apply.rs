use incredible::*;
use incredible_elements::Button;

use crate::state::State;

pub fn build_button_apply() -> Button<State> {
    let button_apply: Button<State> = Button::new();
    button_apply
        .text(" Apply ")
        .x(68)
        .y(17)
        .width(10)
        .focused(false);
    button_apply
        .on_key(|_, state, event| {
            if event.key == Key::Enter && state.is_valid {
                state.should_apply = true;
            }
        })
        .on_mouse(|_, state, event| {
            if event.mouse == Mouse::Click && state.is_valid {
                state.should_apply = true;
            }
        })
        .on_state(|el, state| {
            el.focused(state.focused_index == 6);
            el.disabled(!state.is_valid);
        });
    button_apply
}
