use crate::ui::{APP_HEIGHT, APP_WIDTH};
use crate::State;
use little_tui::*;

// ---------------- Screen ---------------- //
// utility element with no visible look.
// clears screen on resize, "mouse out" of pickers.
pub fn build() -> Element<State> {
    let screen: Element<State> = Element::new();
    screen.look(Look::from((Terminal::columns(), Terminal::rows(), ' ')));

    screen
        .on_loop(|_el, state, _event| {
            let x = (Terminal::columns().saturating_sub(APP_WIDTH) / 2) as isize;
            let y = (Terminal::rows().saturating_sub(APP_HEIGHT) / 2) as isize;

            if x != state.app_x || y != state.app_y {
                Terminal::clear_screen();

                state.app_x = x;
                state.app_y = y;
            }
        })
        .on_mouse(|el, state, event| {
            if event.mouse == Mouse::Move {
                if el.status.hovered.get() {
                    state.picker_mode = false;
                }
            }
        });

    screen
}
