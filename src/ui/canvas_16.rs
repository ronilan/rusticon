use crate::ui::utils::*;
use crate::AppState;
use little_tui::*;

static X: i16 = 23;
static Y: i16 = 3;

pub fn build<'a>() -> Element<AppState> {
    let mut canvas_16: Element<AppState> = Element::new(Pos::new(X, Y), Look::new());

    canvas_16.listener.on_click = Some(Box::new(|el, state, event| {
        if state.size == 16 {
            if event.modifiers.contains(&"ctrl".to_string()) {
                // Handle ctrl-click for color picking
                let row = event.coords.y.get().saturating_sub(el.pos.y.get()) as usize;
                let col = event.coords.x.get().saturating_sub(el.pos.x.get()) as usize / 2;
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
                    event.coords.x.get(),
                    event.coords.y.get(),
                    event.modifiers.contains(&"shift".to_string()),
                );
            }
        }

        let look = canvas_look_from_data(16, &state.canvas16_data);
        el.look.update(look);
        crate::ui::draw_relative(el, X, Y, state);
    }));
    canvas_16.listener.on_state = Some(Box::new(|el, state| {
        if state.size == 16 {
            let look = canvas_look_from_data(16, &state.canvas16_data);
            el.look.update(look);

            crate::ui::draw_relative(el, X, Y, state);
        }
    }));

    canvas_16
}
