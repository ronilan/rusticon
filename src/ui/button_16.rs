use crate::AppState;
use little_tui::*;

static X: u16 = 74;
static Y: u16 = 2;

pub fn build<'a>() -> Element<'a, AppState> {
    let mut button_16: Element<AppState> = Element::new(
        Pos::new(X, Y),
        terminal_style::format::underline(Look::from("16x16")),
    );

    button_16.listener.on_click = Some(Box::new(|_el, state, _event| {
        state.size = 16;
        state.canvas16_data = vec![None; 256];
    }));
    button_16.listener.on_state = Some(Box::new(|el, state| {
        crate::ui::draw_relative(el, X, Y, state);
    }));

    button_16
}
