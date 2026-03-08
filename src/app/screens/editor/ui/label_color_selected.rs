use crate::State;
use little_tui::*;
use little_tui_collection::Text;

static X: isize = 62;
static Y: isize = 8;

pub fn build() -> Text<State> {
    let label_color_selected: Text<State> = Text::default();
    label_color_selected.x(X).y(Y).on_state(|el, state| {
        let text = match state.editor.paintbrush {
            Some(c) => format!("{:<3}   {}", c, Colors::ansi8_to_hex(c)),
            None => format!("{:<13}", ":transparent:"),
        };

        el.text(&text);
    });

    label_color_selected
}
