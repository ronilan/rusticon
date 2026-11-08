use crate::core::model::State;
use incredible::*;
use incredible_elements::Button;
use incredible_helpers_styling::*;

static X: isize = 75;
static Y: isize = 19;

pub fn build() -> Button<State> {
    let button_save: Button<State> = Button::default();
    button_save.conf_text();
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
