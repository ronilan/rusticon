use crate::{ui::reposition, State};
use incredible::*;

static X: isize = 67;
static Y: isize = 19;

pub fn build() -> Element<State> {
    let button_exit: Element<State> = Element::new();
    button_exit
        .x(X)
        .y(Y)
        .look(Look::from("Exit"))
        .on_mouse(|_el, _state, event| {
            if event.mouse == Mouse::Click {
                exit();
            }
        })
        .on_state(|el, state| {
            reposition(el, X, Y, state);
            el.decorate();
            el.draw();
        });

    button_exit
}
