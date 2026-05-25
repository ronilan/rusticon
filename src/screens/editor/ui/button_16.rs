use crate::core::model::State;
use incredible::*;
use incredible_elements::TextButton;

static X: isize = 74;
static Y: isize = 2;

pub fn build() -> TextButton<State> {
    let button_16: TextButton<State> = TextButton::default();
    button_16
        .x(X)
        .y(Y)
        .text("16x16")
        .underline(true)
        .on_mouse(|_el, state, event| {
            if event.mouse == Mouse::Click {
                state.editor.size = 16;
                state.editor.canvas16_data = vec![None; 256];
            }
        });

    button_16
}
