use super::utils::*;
use crate::core::model::AppPhase;
use crate::core::model::State;
use incredible::*;

static X: isize = 23;
static Y: isize = 3;

pub fn build() -> Element<State> {
    let canvas_16: Element<State> = Element::new();
    canvas_16
        .x(X)
        .y(Y)
        .on_mouse(|el, state, event| {
            // If mouse moves, it will not be a double click fill.
            if event.mouse == Mouse::Move {
               state.editor.prev_color_on_canvas = None;
            }
            let is_paint = event.mouse == Mouse::Down || event.mouse == Mouse::Drag;
            let is_fill = event.mouse == Mouse::DoubleClick;
            if !(is_paint || is_fill) {
                return;
            }
            if state.editor.size != 16 {
                return;
            }

            if event.modifiers.contains(&KeyMod::Ctrl) {
                // Handle ctrl-click for color picking (not on double-click fill)
                if is_paint {
                    let row = event.y.saturating_sub(el.visual.y.get()) as usize;
                    let col = event.x.saturating_sub(el.visual.x.get()) as usize / 2;
                    if row < 16 && col < 16 {
                        state.editor.paintbrush = state.editor.canvas16_data[row * 16 + col];
                        set_palette_in_state(state, state.editor.paintbrush);
                    }
                }
            } else {
                let paintbrush = state.editor.paintbrush;
                canvas_data_from_click(
                    el,
                    16,
                    &mut state.editor.canvas16_data,
                    paintbrush,
                    event.x,
                    event.y,
                    &mut state.editor.prev_color_on_canvas,
                    is_fill,
                );
            }
            let look = canvas_look_from_data(16, &state.editor.canvas16_data);
            el.look(look);
        })
        .on_state(|el, state| {
            let active = state.flow.phase == AppPhase::Main && state.editor.size == 16;
            if active {
                el.showed(true);
                let look = canvas_look_from_data(16, &state.editor.canvas16_data);
                el.look(look);
                el.draw();
                return;
            }

            // On hide, draw a blank patch once at the same location to clear stale pixels.
            if el.get_showed() {
                el.showed(true);
                el.look(Look::from((32, 16, ' ')));
                el.draw();
            }
            el.showed(false);
        });

    canvas_16
}
