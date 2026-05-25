use super::utils::*;
use crate::core::model::AppPhase;
use crate::core::model::State;
use incredible::*;

static X: isize = 31;
static Y: isize = 7;

pub fn build() -> Element<State> {
    let canvas_8: Element<State> = Element::new();
    canvas_8
        .x(X)
        .y(Y)
        .on_mouse(|el, state, event| {
            if event.mouse == Mouse::Down || event.mouse == Mouse::Drag {
                if state.editor.size == 8 {
                    if event.modifiers.contains(&KeyMod::Ctrl) {
                        // Handle ctrl-click for color picking
                        let row = event.y.saturating_sub(el.visual.y.get()) as usize;
                        let col = event.x.saturating_sub(el.visual.x.get()) as usize / 2;
                        if row < 8 && col < 8 {
                            state.editor.paintbrush = state.editor.canvas8_data[row * 8 + col];
                            set_palette_in_state(state, state.editor.paintbrush);
                        }
                    } else {
                        canvas_data_from_click(
                            el,
                            8,
                            &mut state.editor.canvas8_data,
                            state.editor.paintbrush,
                            event.x,
                            event.y,
                            event.modifiers.contains(&KeyMod::Shift),
                        );
                    }

                    let look = canvas_look_from_data(8, &state.editor.canvas8_data);
                    el.look(look);
                }
            }
        })
        .on_state(|el, state| {
            let active = state.flow.phase == AppPhase::Main && state.editor.size == 8;
            el.showed(active);
            if !active {
                return;
            }

            let look = canvas_look_from_data(8, &state.editor.canvas8_data);
            el.look(look);
            el.draw();
        });

    canvas_8
}
