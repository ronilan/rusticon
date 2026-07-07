use crate::ui::utils::*;
use crate::{ui::reposition, State};
use incredible::*;

static X: isize = 23;
static Y: isize = 3;

pub fn build() -> Element<State> {
    let canvas_16: Element<State> = Element::new();
    canvas_16
        .x(X)
        .y(Y)
        .on_mouse(|el, state, event| {
            if event.mouse == Mouse::Down || event.mouse == Mouse::Drag {
                if state.size == 16 {
                    if event.modifiers.contains(&KeyMod::Ctrl) {
                        // Handle ctrl-click for color picking
                        let row = event.y.saturating_sub(el.visual.y.get()) as usize;
                        let col = event.x.saturating_sub(el.visual.x.get()) as usize / 2;
                        if row < 16 && col < 16 {
                            state.paintbrush = state.canvas16_data[row * 16 + col];
                            set_palette_in_state(state, state.paintbrush);
                        }
                    } else {
                        canvas_data_from_click(
                            el,
                            16,
                            &mut state.canvas16_data,
                            state.paintbrush,
                            event.x,
                            event.y,
                            event.modifiers.contains(&KeyMod::Shift),
                        );
                    }
                    let look = canvas_look_from_data(16, &state.canvas16_data);
                    el.look(look);
                    reposition(el, X, Y, state);
                }
            }
        })
        .on_state(|el, state| {
            if state.size == 16 {
                let look = canvas_look_from_data(16, &state.canvas16_data);
                el.look(look);

                reposition(el, X, Y, state);
                el.draw();
            }
        });

    canvas_16
}
