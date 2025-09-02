use crate::tui_engine::*;
use crate::ui::{APP_HEIGHT, APP_WIDTH};
use crate::AppState;

// ---------------- Screen ---------------- //
// utility element with no visible look.
// clears screen on resize, "mouse out" of pickers.
pub fn build<'a>() -> Element<'a, AppState> {
    let mut screen: Element<AppState> = Element::new(0, 0, Look::new());

    screen.on_loop = Some(Box::new(|_el, state, _event| {
        let x = columns().saturating_sub(APP_WIDTH) / 2;
        let y = rows().saturating_sub(APP_HEIGHT) / 2;

        if x != state.app_x || y != state.app_y {
            go_to(0, 1);
            clear_below();

            state.app_x = x;
            state.app_y = y;
        }
    }));
    screen.on_move = Some(Box::new(|_el, state, _event| {
        state.picker_mode = false;
    }));
    
    screen
}
