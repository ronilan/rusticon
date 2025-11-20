use crate::AppState;
use little_tui::*;

static X: i16 = 62;
static Y: i16 = 8;

pub fn build() -> Element<AppState> {
    let mut label_color_selected: Element<AppState> = Element::new(Pos::new(X, Y), Look::new());

    label_color_selected.listener.on_state = |el, state| {
        let text = match state.paintbrush {
            Some(c) => format!("{:<3}   {}", c, terminal_style::color::ansi8_to_hex(c)),
            None => format!("{:<13}", ":transparent:"),
        };

        el.look.set(text);
        crate::ui::draw_relative(el, X, Y, state);
    };

    label_color_selected
}
