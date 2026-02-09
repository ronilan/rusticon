use crate::ui::utils::*;
use crate::{ui::reposition, AppState};
use little_tui::*;

static X: isize = 16;
static Y: isize = 18;

pub fn build() -> Element<AppState> {
    let color_picker_empty: Element<AppState> = Element::new();
    color_picker_empty
        .x(X)
        .y(Y)
        .look(Look::from("::\n::\n"))
        .on_mouse(|_el, state, event| {
            if event.mouse == Mouse::Move || event.mouse == Mouse::Click {
                state.candidate = None;

                if event.mouse == Mouse::Move {
                    state.picker_mode = true;
                }
                if event.mouse == Mouse::Click {
                    state.paintbrush = None;
                    set_palette_in_state(state, state.candidate);
                }
            }
        })
        .on_state(|el, state| {
            reposition(el, X, Y, state);
            el.draw();
        });

    color_picker_empty
}
