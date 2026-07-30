use incredible::*;
use incredible_elements::Button;

use crate::state::State;

pub fn build_button_package() -> Button<State> {
    let button: Button<State> = Button::new();
    button
        .text(" Package ")
        .x(68)
        .y(4)
        .width(11)
        .focused(false)
        .disabled(true);

    button
        .on_key(|_, state, event| {
            if event.key == Key::Enter {
                state.should_execute = true;
            }
        })
        .on_mouse(|_, state, event| {
            if event.mouse == Mouse::Click {
                state.focused_index = 2;
                state.should_execute = true;
            }
        })
        .on_state(|el, state| {
            el.focused(state.focused_index == 3);
            el.disabled(state.selected_target.is_none());
        });

    button
}
