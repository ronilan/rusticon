use crate::{ui::reposition, State};
use incredible::*;

static X: isize = 59;
static Y: isize = 2;

pub fn build() -> Element<State> {
    let label_new: Element<State> = Element::new();
    label_new
        .x(X)
        .y(Y)
        .look(Look::from("New:"))
        .on_state(|el, state| {
            reposition(el, X, Y, state);
            el.draw();
        });

    label_new
}
