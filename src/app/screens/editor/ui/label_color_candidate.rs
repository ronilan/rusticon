use crate::{ui::reposition, State};
use incredible::*;

static X: isize = 62;
static Y: isize = 13;

pub fn build() -> Element<State> {
    let label_color_candidate: Element<State> = Element::new();
    label_color_candidate
        .x(X)
        .y(Y)
        .look(Look::new())
        .on_state(|el, state| {
            let text = if state.picker_mode {
                match state.candidate {
                    Some(c) => format!("{:<3}   {}", c, Colors::ansi8_to_hex(c)),
                    None => format!("{:<13}", ":transparent:"),
                }
            } else {
                "             ".to_string()
            };

            el.look(Look::from(text));

            reposition(el, X, Y, state);
            el.draw();
        });

    label_color_candidate
}
