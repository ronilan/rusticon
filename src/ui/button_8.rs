use crate::tui_engine::*;
use crate::AppState;

const X: u16 = 67;
const Y: u16 = 2;

pub fn build<'a>() -> Element<'a, AppState> {
    let mut button_8: Element<AppState> =
        Element::new(X, Y, terminal_style::format::underline(Look::from("8x8")));

    button_8.on_click = Some(Box::new(|el, state, event| {
        if mouse_over(el, event) {
            state.size = 8;
            state.canvas8_data = vec![None; 64];

            // erase the 16x16 area
            let eraser: Element<AppState> = Element::new(
                state.app_x + 23,
                state.app_y + 3,
                Look::from(vec![vec![" ".to_string(); 32]; 16]),
            );
            crate::ui::draw_relative(&eraser, X, Y, state);
        }
    }));
    button_8.on_state = Some(Box::new(|el, state| {
        el.x.set(state.app_x + 67);
        el.y.set(state.app_y + 2);
        crate::ui::draw_relative(el, X, Y, state);
    }));

    button_8
}
