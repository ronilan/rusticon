use incredible::*;
use incredible_elements::Button;

use crate::state::State;

pub fn build_button_quit() -> Button<State> {
    let button_quit: Button<State> = Button::new();
    button_quit
        .text(" Quit ")
        .x(68)
        .y(14)
        .width(10)
        .focused(false);
    button_quit
        .on_key(|_, state, event| {
            if event.key == Key::Enter {
                state.should_quit = true;
            }
        })
        .on_mouse(|_, state, event| {
            if event.mouse == Mouse::Click {
                state.should_quit = true;
            }
        })
        .on_state(|el, state| {
            el.focused(state.focused_index == 5);
        });
    button_quit
}
