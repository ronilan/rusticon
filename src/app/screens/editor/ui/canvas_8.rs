use crate::ui::utils::*;
use crate::{ui::reposition, State};
use incredible::*;

static X: isize = 31;
static Y: isize = 7;

pub fn build() -> Element<State> {
    let canvas_8: Element<State> = Element::new();
    canvas_8
        .x(X)
        .y(Y)
        .on_mouse(|el, state, event| {
            // If mouse moves, it will not be a double click fill.
            if event.mouse == Mouse::Move {
                state.prev_color_on_canvas = None;
            }
            let is_paint = event.mouse == Mouse::Down || event.mouse == Mouse::Drag;
            let is_fill = event.mouse == Mouse::DoubleClick;
            if !(is_paint || is_fill) {
                return;
            }
            if state.size != 8 {
                return;
            }

            if event.modifiers.contains(&KeyMod::Ctrl) {
                // Handle ctrl-click for color picking (not on double-click fill)
                if is_paint {
                    let row = event.y.saturating_sub(el.visual.y.get()) as usize;
                    let col = event.x.saturating_sub(el.visual.x.get()) as usize / 2;
                    if row < 8 && col < 8 {
                        state.paintbrush = state.canvas8_data[row * 8 + col];
                        set_palette_in_state(state, state.paintbrush);
                    }
                }
            } else {
                let paintbrush = state.paintbrush;
                canvas_data_from_click(
                    el,
                    8,
                    &mut state.canvas8_data,
                    paintbrush,
                    event.x,
                    event.y,
                    &mut state.prev_color_on_canvas,
                    is_fill,
                );
            }

            let look = canvas_look_from_data(8, &state.canvas8_data);
            el.look(look);
            reposition(el, X, Y, state);
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
