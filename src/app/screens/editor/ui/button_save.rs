use crate::{ui::reposition, State};
use incredible::*;

static X: isize = 75;
static Y: isize = 19;

pub fn build() -> Element<State> {
    let button_save: Element<State> = Element::new();
    button_save
        .x(X)
        .y(Y)
        .look(Look::from("Save"))
        .on_loop(|_el, state, _event| {
            if state.save_flag {
                // wait till next loop to exit
                exit();
            }
        })
        .on_mouse(|_el, state, event| {
            if event.mouse == Mouse::Click {
                state.save_flag = true;
            }
        })
        .on_state(|el, state| {
            reposition(el, X, Y, state);
            el.decorate();
            el.draw();
        });

    button_save
}
