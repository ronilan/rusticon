use incredible::*;
use incredible_elements::Button;

use crate::state::State;

pub fn build_button_quit() -> Button<State> {
    let button: Button<State> = Button::new();
    button.text(" Quit ").x(68).y(1).width(11).focused(false);

    button
        .on_key(|_, _state, event| {
            if event.key == Key::Enter {
                exit();
            }
        })
        .on_mouse(|_, _state, event| {
            if event.mouse == Mouse::Click {
                exit();
            }
        })
        .on_state(|el, state| {
            el.focused(state.focused_index == 2);
        });

    button
}
