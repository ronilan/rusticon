use crate::AppState;
use little_tui::*;

static X: isize = 74;
static Y: isize = 2;

pub fn build() -> Element<AppState> {
    let button_16: Element<AppState> = Element::new();
    button_16
        .x(X)
        .y(Y)
        .look(Look::from("16x16"))
        .underline(true)
        .on_mouse(|_el, state, event: &EventMouse| {
            if event.mouse == Mouse::Click {
                state.size = 16;
                state.canvas16_data = vec![None; 256];
            }
        })
        .on_state(|el, state| {
            el.decorate();
            crate::ui::draw_relative(el, X, Y, state);
        });

    button_16
}
