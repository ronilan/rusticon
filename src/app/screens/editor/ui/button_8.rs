use crate::State;
use little_tui::*;
use little_tui_collection::TextButton;

static X: isize = 67;
static Y: isize = 2;

pub fn build() -> TextButton<State> {
    let button_8: TextButton<State> = TextButton::default();
    button_8
        .x(X)
        .y(Y)
        .text("8x8")
        .underline(true)
        .on_mouse(|_el, state, event| {
            if event.mouse == Mouse::Click {
                state.editor.size = 8;
                state.editor.canvas8_data = vec![None; 64];
            }
        });

    button_8
}
