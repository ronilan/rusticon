use crate::State;
use little_tui::*;
use little_tui_collection::Text;

static X: isize = 62;
static Y: isize = 13;

pub fn build() -> Text<State> {
    let label_color_candidate: Text<State> = Text::default();
    label_color_candidate
        .x(X)
        .y(Y)
        .text("")
        .on_state(|el, state| {
            let text = if state.editor.picker_mode {
                match state.editor.candidate {
                    Some(c) => format!("{:<3}   {}", c, terminal_style::color::ansi8_to_hex(c)),
                    None => format!("{:<13}", ":transparent:"),
                }
            } else {
                "             ".to_string()
            };

            el.text(&text);
        });

    label_color_candidate
}
