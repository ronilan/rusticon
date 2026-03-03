use crate::{ui::reposition, State};
use little_tui::*;

static X: isize = 59;
static Y: isize = 19;

pub fn build() -> Element<State> {
    let label_end: Element<State> = Element::new();
    label_end.x(X).y(Y).look(Look::from("End:"));

    label_end.on_state(|el, state| {
        reposition(el, X, Y, state);
        el.draw();
    });

    label_end
}
