use incredible::*;
use incredible_elements::Button;

use crate::state::State;

pub fn build_button_quit() -> Button<State> {
    let button_quit: Button<State> = Button::new();
    button_quit
        .text(" Quit ")
        .x(68)
        .y(13)
        .width(10)
        .focused(false);

    button_quit
        .on_key(|_el, state, event| {
            if event.key == Key::Enter {
                state.should_quit = true;
            }
        })
        .on_mouse(|_el, state, event| {
            if event.mouse == Mouse::Click {
                state.should_quit = true;
            }
        })
        .on_change(|el, state, _event| {
            if el.status().focused.get() {
                state.focused_index = 5;
            }
        });
    button_quit
}
