use crate::AppState;
use little_tui::*;

static X: i16 = 67;
static Y: i16 = 19;

pub fn build() -> Element<AppState> {
    let mut button_exit: Element<AppState> = Element::new();
    button_exit
        .x(X)
        .y(Y)
        .look(terminal_style::format::underline(Look::from("Exit")));

    button_exit.listener.on_mouse = |_el, _state, event| {
        if event.mouse == Mouse::Click {
            exit();
        }
    };
    button_exit.listener.on_state = |el, state| {
        crate::ui::draw_relative(el, X, Y, state);
    };

    button_exit
}
