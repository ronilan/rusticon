use crate::core::model::State;
use little_tui::*;
use little_tui_elements::TextButton;

static X: isize = 75;
static Y: isize = 19;

pub fn build() -> TextButton<State> {
    let button_save: TextButton<State> = TextButton::default();
    button_save
        .x(X)
        .y(Y)
        .text("Save")
        .underline(true)
        .on_mouse(|_el, state, event| {
            if event.mouse == Mouse::Click {
                state.editor.save_requested = true;
            }
        });

    button_save
}
