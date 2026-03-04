use crate::State;
use little_tui::*;
use little_tui_collection::TextButton;

static X: isize = 67;
static Y: isize = 19;

pub fn build() -> TextButton<State> {
    let button_exit: TextButton<State> = TextButton::default();
    button_exit
        .x(X)
        .y(Y)
        .text("Exit")
        .underline(true)
        .on_mouse(|_el, _state, event| {
            if event.mouse == Mouse::Click {
                exit();
            }
        });

    button_exit
}
