use crate::ui::utils::*;
use crate::{ui::reposition, AppState};
use little_tui::*;

static X: isize = 31;
static Y: isize = 7;

pub fn build() -> Element<AppState> {
    let canvas_8: Element<AppState> = Element::new();
    canvas_8
        .x(X)
        .y(Y)
        .on_mouse(|el, state, event| {
            if event.mouse == Mouse::Down || event.mouse == Mouse::Drag {
                if state.size == 8 {
                    if event.modifiers.contains(&KeyMod::Ctrl) {
                        // Handle ctrl-click for color picking
                        let row = event.y.saturating_sub(el.visual.y.get()) as usize;
                        let col = event.x.saturating_sub(el.visual.x.get()) as usize / 2;
                        if row < 8 && col < 8 {
                            state.paintbrush = state.canvas8_data[row * 8 + col];
                            set_palette_in_state(state, state.paintbrush);
                        }
                    } else {
                        canvas_data_from_click(
                            el,
                            8,
                            &mut state.canvas8_data,
                            state.paintbrush,
                            event.x,
                            event.y,
                            event.modifiers.contains(&KeyMod::Shift),
                        );
                    }

                    let look = canvas_look_from_data(8, &state.canvas8_data);
                    el.look(look);
                    reposition(el, X, Y, state);
                }
            }
        })
        .on_state(|el, state| {
            if state.size == 8 {
                let look = canvas_look_from_data(8, &state.canvas8_data);
                el.look(look);

                reposition(el, X, Y, state);
                el.draw();
            }
        });

    canvas_8
}
