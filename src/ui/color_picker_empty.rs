use crate::ui::utils::*;
use crate::AppState;
use little_tui::*;

static X: i16 = 16;
static Y: i16 = 18;

pub fn build() -> Element<AppState> {
    let mut color_picker_empty: Element<AppState> = Element::new(
        Pos::new(X, Y),
        Look::from(vec![
            vec![":".to_string(), ":".to_string()],
            vec![":".to_string(), ":".to_string()],
        ]),
    );

    color_picker_empty.listener.on_mouse = |_el, state, event| {
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
    };
    color_picker_empty.listener.on_state = |el, state| {
        crate::ui::draw_relative(el, X, Y, state);
    };

    color_picker_empty
}
