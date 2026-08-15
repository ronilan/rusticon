use incredible::*;
use incredible_elements::Button;

use crate::state::State;

pub fn build_button_quit() -> Button<State> {
    let button: Button<State> = Button::new();
    button
        .text(" Quit ")
        .x(68)
        .y(1)
        .width(11)
        .key_combination(Some(KeyCombination::new(Key::Enter, &[])))
        .focused(false);

    button
        .on_key(|_, _state, event| {
            if event.key == Key::Enter {
                exit(0);
            }
        })
        .on_mouse(|_, _state, event| {
            if event.mouse == Mouse::Click {
                exit(0);
            }
        });

    button
}
