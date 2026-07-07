use crate::{ui::reposition, State};
use incredible::*;

static X: isize = 62;
static Y: isize = 8;

pub fn build() -> Element<State> {
    let label_color_selected: Element<State> = Element::new();
    label_color_selected.x(X).y(Y).on_state(|el, state| {
        let text = match state.paintbrush {
            Some(c) => format!("{:<3}   {}", c, Colors::ansi8_to_hex(c)),
            None => format!("{:<13}", ":transparent:"),
        };

        el.look(Look::from(text));
        reposition(el, X, Y, state);
        el.draw();
    });

    label_color_selected
}
