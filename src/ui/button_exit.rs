use crate::{ui::reposition, AppState};
use little_tui::*;

static X: isize = 67;
static Y: isize = 19;

pub fn build() -> Element<AppState> {
    let button_exit: Element<AppState> = Element::new();
    button_exit
        .x(X)
        .y(Y)
        .look(Look::from("Exit"))
        .underline(true)
        .on_mouse(|_el, _state, event| {
            if event.mouse == Mouse::Click {
                exit();
            }
        })
        .on_state(|el, state| {
            reposition(el, X, Y, state);
            el.decorate();
            el.draw();
        });

    button_exit
}
