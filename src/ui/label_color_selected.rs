use crate::tui_engine::*;
use crate::AppState;

const X: u16 = 62;
const Y: u16 = 8;

pub fn build<'a>() -> Element<'a, AppState> {
    let mut label_color_selected: Element<AppState> = Element::new(X, Y, Look::new());

    label_color_selected.on_state = Some(Box::new(|el, state| {
        let text = match state.paintbrush {
            Some(c) => format!("{:<3}   {}", c, terminal_style::color::ansi8_to_hex(c)),
            None => format!("{:<13}", ":transparent:"),
        };

        el.look.update(text);
        crate::ui::draw_relative(el, X, Y, state);
    }));

    label_color_selected
}
