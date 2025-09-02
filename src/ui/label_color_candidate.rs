use crate::tui_engine::*;
use crate::AppState;

const X: u16 = 62;
const Y: u16 = 13;

pub fn build<'a>() -> Element<'a, AppState> {
    let mut label_color_candidate: Element<AppState> =
        Element::new(X, Y, Look::from(vec![vec!["".to_string()]]));

    label_color_candidate.on_state = Some(Box::new(|el, state| {
        let text = if state.picker_mode {
            match state.candidate {
                Some(c) => format!("{:<3}   {}", c, terminal_style::color::ansi8_to_hex(c)),
                None => format!("{:<13}", ":transparent:"),
            }
        } else {
            "             ".to_string()
        };

        el.look.update(text);
        crate::ui::draw_relative(el, X, Y, state);
    }));

    label_color_candidate
}
