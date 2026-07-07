use crate::{ui::reposition, State};
use incredible::*;

static X: isize = 74;
static Y: isize = 2;

pub fn build() -> Element<State> {
    let button_16: Element<State> = Element::new();
    button_16
        .x(X)
        .y(Y)
        .look(Look::from("16x16"))
        .on_mouse(|_el, state, event: &EventMouse| {
            if event.mouse == Mouse::Click {
                state.size = 16;
                state.canvas16_data = vec![None; 256];
            }
        })
        .on_state(|el, state| {
            reposition(el, X, Y, state);
            el.decorate();
            el.draw();
        });

    button_16
}
