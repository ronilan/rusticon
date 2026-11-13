use crate::core::model::{ExitFlow, State};
use incredible::*;
use incredible_elements::Button;
use incredible_helpers_styling::*;

static X: isize = 67;
static Y: isize = 19;

pub fn build() -> Button<State> {
    let button_exit: Button<State> = Button::default();
    button_exit.conf_text();
    button_exit
        .x(X)
        .y(Y)
        .focused(false)
        .text("Exit")
        .underline(Some(UnderlineKind::Dotted))
        .on_mouse(|_el, state, event| {
            if event.mouse == Mouse::Click {
                state.flow.exit_flow = ExitFlow::ExitRequested;
            }
        });

    button_exit
}
