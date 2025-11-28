use crate::ui::{APP_HEIGHT, APP_WIDTH};
use crate::AppState;
use little_tui::*;

// ---------------- Screen ---------------- //
// utility element with no visible look.
// clears screen on resize, "mouse out" of pickers.
pub fn build() -> Element<AppState> {
    let mut screen: Element<AppState> = Element::new();
    screen.look(Look::from((columns(), rows(), ' ')));

    screen.listener.on_loop = |_el, state, _event| {
        let x = (columns().saturating_sub(APP_WIDTH) / 2) as i16;
        let y = (rows().saturating_sub(APP_HEIGHT) / 2) as i16;

        if x != state.app_x || y != state.app_y {
            clear_screen();

            state.app_x = x;
            state.app_y = y;
        }
    };
    screen.listener.on_mouse = |_el, state, event| {
        if event.mouse == Mouse::Move {
            state.picker_mode = false;
        }
    };

    screen
}
