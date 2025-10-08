use crate::ui::utils::*;
use crate::AppState;
use little_tui::*;

static X: i16 = 31;
static Y: i16 = 7;

pub fn build<'a>() -> Element<AppState> {
    let mut canvas_8: Element<AppState> = Element::new(Pos::new(X, Y), Look::new());

    canvas_8.listener.on_click = Some(Box::new(|el, state, event| {
        if state.size == 8 {
            if event.modifiers.contains(&"ctrl".to_string()) {
                // Handle ctrl-click for color picking
                let row = event.pos.y.get().saturating_sub(el.pos.y.get()) as usize;
                let col = event.pos.x.get().saturating_sub(el.pos.x.get()) as usize / 2;
                if row < 8 && col < 8 {
                    state.paintbrush = state.canvas8_data[row * 8 + col];
                    state.candidate = state.paintbrush;
                    set_palette_in_state(state, state.candidate);
                }
            } else {
                canvas_data_from_click(
                    el,
                    8,
                    &mut state.canvas8_data,
                    state.paintbrush,
                    event.pos.x.get(),
                    event.pos.y.get(),
                    event.modifiers.contains(&"shift".to_string()),
                );
            }
        }

        let look = canvas_look_from_data(8, &state.canvas8_data);
        el.look.update(look);
        crate::ui::draw_relative(el, X, Y, state);
    }));
    canvas_8.listener.on_state = Some(Box::new(|el, state| {
        if state.size == 8 {
            let look = canvas_look_from_data(8, &state.canvas8_data);
            el.look.update(look);

            crate::ui::draw_relative(el, X, Y, state);
        }
    }));

    canvas_8
}
