use crate::core::model::State;
use incredible::*;
use incredible_elements::Button;
use incredible_helpers_styling::*;

static X: isize = 74;
static Y: isize = 2;

pub fn build() -> Button<State> {
    let button_16: Button<State> = Button::default();
    button_16.conf_text();
    button_16
        .x(X)
        .y(Y)
        .focused(false)
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
