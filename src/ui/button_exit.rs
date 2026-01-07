use crate::AppState;
use little_tui::*;

static X: i16 = 67;
static Y: i16 = 19;

pub fn build() -> Element<AppState> {
    let button_exit: Element<AppState> = Element::new();
    button_exit
        .x(X)
        .y(Y)
        .look(Look::from("Exit"))
        .underline(true).on_mouse(|_el, _state, event| {
        if event.mouse == Mouse::Click {
            exit();
        }
    }).on_state(|el, state| {
        el.decorate();
        crate::ui::draw_relative(el, X, Y, state);
    });

    button_exit
}
