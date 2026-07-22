use crate::core::model::State;
use incredible::*;
use incredible_elements::TextButton;
use incredible_helpers_styling::*;

static X: isize = 74;
static Y: isize = 2;

pub fn build() -> TextButton<State> {
    let button_16: TextButton<State> = TextButton::default();
    button_16
        .x(X)
        .y(Y)
        .focused(false)
        .pointer(Some(PointerShape::Pointer))
        .text("16x16")
        .underline(Some(UnderlineKind::Dotted))
        .on_mouse(|_el, state, event| {
            if event.mouse == Mouse::Click {
                state.editor.size = 16;
                state.editor.canvas16_data = vec![None; 256];
            }
        });

    button_16
}
