use crate::tui_engine::*;
use crate::AppState;

const X: u16 = 74;
const Y: u16 = 2;

pub fn build<'a>() -> Element<'a, AppState> {
    let mut button_16: Element<AppState> =
        Element::new(X, Y, terminal_style::format::underline(Look::from("16x16")));

    button_16.on_click = Some(Box::new(|el, state, event| {
        if mouse_over(el, event) {
            state.size = 16;
            state.canvas16_data = vec![None; 256];
        }
    }));
    button_16.on_state = Some(Box::new(|el, state| {
        crate::elements::draw_relative(el, X, Y, state);
    }));

    button_16
}
