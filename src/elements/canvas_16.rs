use crate::elements::utils::*;
use crate::tui_engine::*;
use crate::AppState;

static X: u16 = 23;
static Y: u16 = 3;

pub fn build<'a>() -> Element<'a, AppState> {
    let mut canvas_16: Element<AppState> = Element::new(0, 0, Look::new());

    canvas_16.on_click = Some(Box::new(|el, state, event| {
        if state.size == 16 {
            if mouse_over(el, event) {
                if event.modifiers.contains(&"ctrl".to_string()) {
                    // Handle ctrl-click for color picking
                    let row = event.y.unwrap().saturating_sub(el.y.get()) as usize;
                    let col = event.x.unwrap().saturating_sub(el.x.get()) as usize / 2;
                    if row < 16 && col < 16 {
                        state.paintbrush = state.canvas16_data[row * 16 + col];
                        state.candidate = state.paintbrush;
                        set_palette_in_state(state, state.candidate);
                    }
                } else {
                    canvas_data_from_click(
                        el,
                        16,
                        &mut state.canvas16_data,
                        state.paintbrush,
                        event.x.unwrap(),
                        event.y.unwrap(),
                        event.modifiers.contains(&"shift".to_string()),
                    );
                }
            }

            let look = canvas_look_from_data(16, &state.canvas16_data);
            el.look.update(look);
            crate::elements::draw_relative(el, X, Y, state);
        }
    }));
    canvas_16.on_state = Some(Box::new(|el, state| {
        if state.size == 16 {
            let look = canvas_look_from_data(16, &state.canvas16_data);
            el.look.update(look);

            crate::elements::draw_relative(el, X, Y, state);
        }
    }));

    canvas_16
}
