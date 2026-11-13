use crate::core::model::State;
use incredible::*;
use incredible_elements::Button;
use incredible_helpers_styling::*;

static X: isize = 67;
static Y: isize = 2;

pub fn build() -> Button<State> {
    let button_8: Button<State> = Button::default();
    button_8.conf_text();
    button_8
        .x(X)
        .y(Y)
        .focused(false)
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
