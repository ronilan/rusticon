use crate::core::model::State;
use incredible::*;
use incredible_elements::TextButton;
use incredible_helpers_styling::*;

static X: isize = 67;
static Y: isize = 2;

pub fn build() -> TextButton<State> {
    let button_8: TextButton<State> = TextButton::default();
    button_8
        .x(X)
        .y(Y)
        .focused(false)
        .pointer(Some(PointerShape::Pointer))
        .text("8x8")
        .underline(Some(UnderlineKind::Dotted))
        .on_mouse(|_el, state, event| {
            if event.mouse == Mouse::Click {
                state.editor.size = 8;
                state.editor.canvas8_data = vec![None; 64];
            }
        });

    button_8
}
