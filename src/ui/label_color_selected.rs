use crate::{ui::reposition, AppState};
use little_tui::*;

static X: isize = 62;
static Y: isize = 8;

pub fn build() -> Element<AppState> {
    let label_color_selected: Element<AppState> = Element::new();
    label_color_selected.x(X).y(Y).on_state(|el, state| {
        let text = match state.paintbrush {
            Some(c) => format!("{:<3}   {}", c, terminal_style::color::ansi8_to_hex(c)),
            None => format!("{:<13}", ":transparent:"),
        };

        el.look(Look::from(text));
        reposition(el, X, Y, state);
        el.draw();
    });

    label_color_selected
}
