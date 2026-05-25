use crate::core::model::{ExitFlow, State};
use incredible::*;
use incredible_elements::TextButton;

static X: isize = 67;
static Y: isize = 19;

pub fn build() -> TextButton<State> {
    let button_exit: TextButton<State> = TextButton::default();
    button_exit
        .x(X)
        .y(Y)
        .text("Exit")
        .underline(true)
        .on_mouse(|_el, state, event| {
            if event.mouse == Mouse::Click {
                state.flow.exit_flow = ExitFlow::ExitRequested;
            }
        });

    button_exit
}
