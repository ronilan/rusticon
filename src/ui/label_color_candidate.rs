use crate::AppState;
use little_tui::*;

static X: i16 = 62;
static Y: i16 = 13;

pub fn build() -> Element<AppState> {
    let mut label_color_candidate: Element<AppState> =
        Element::new(Pos::new(X, Y), Look::from(vec![vec!["".to_string()]]));

    label_color_candidate.listener.on_state = |el, state| {
        let text = if state.picker_mode {
            match state.candidate {
                Some(c) => format!("{:<3}   {}", c, terminal_style::color::ansi8_to_hex(c)),
                None => format!("{:<13}", ":transparent:"),
            }
        } else {
            "             ".to_string()
        };

        el.look.set(text);
        crate::ui::draw_relative(el, X, Y, state);
    };

    label_color_candidate
}
