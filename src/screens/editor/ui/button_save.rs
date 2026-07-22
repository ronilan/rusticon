use crate::core::model::State;
use incredible::*;
use incredible_elements::TextButton;
use incredible_helpers_styling::*;

static X: isize = 75;
static Y: isize = 19;

pub fn build() -> TextButton<State> {
    let button_save: TextButton<State> = TextButton::default();
    button_save
        .x(X)
        .y(Y)
        .focused(false)
        .pointer(Some(PointerShape::Pointer))
        .text("Save")
        .underline(Some(UnderlineKind::Dotted))
        .on_mouse(|_el, state, event| {
            if event.mouse == Mouse::Click {
                state.editor.save_requested = true;
            }
        });

    button_save
}
