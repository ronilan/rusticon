use super::utils::*;
use crate::core::model::State;
use incredible::*;
use incredible_helpers_styling::*;

static X: isize = 16;
static Y: isize = 18;

pub fn build() -> Element<State> {
    let color_picker_empty: Element<State> = Element::new();
    color_picker_empty
        .x(X)
        .y(Y)
        .pointer(Some(PointerShape::Crosshair))
        .look(Look::from("::\n::\n"))
        .on_mouse(|_el, state, event| {
            if event.mouse == Mouse::Move || event.mouse == Mouse::Click {
                state.editor.candidate = None;

                if event.mouse == Mouse::Move {
                    state.editor.picker_mode = true;
                }
                if event.mouse == Mouse::Click {
                    state.editor.paintbrush = None;
                    set_palette_in_state(state, state.editor.candidate);
                }
            }
        })
        .on_state(|el, _state| {
            el.draw();
        });

    color_picker_empty
}
