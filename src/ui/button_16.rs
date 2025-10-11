use crate::AppState;
use little_tui::*;

static X: i16 = 74;
static Y: i16 = 2;

pub fn build() -> Element<AppState> {
    let mut button_16: Element<AppState> = Element::new(
        Pos::new(X, Y),
        terminal_style::format::underline(Look::from("16x16")),
    );

    button_16.listener.on_mouse = Some(Box::new(|_el, state, event| {
        if event.kind == "click" {
            state.size = 16;
            state.canvas16_data = vec![None; 256];
        }
    }));
    button_16.listener.on_state = Some(Box::new(|el, state| {
        crate::ui::draw_relative(el, X, Y, state);
    }));

    button_16
}
