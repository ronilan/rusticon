use crate::ui::utils::*;
use crate::AppState;
use little_tui::*;

static X: i16 = 31;
static Y: i16 = 7;

pub fn build() -> Element<AppState> {
    let mut canvas_8: Element<AppState> = Element::new(Pos::new(X, Y), Look::new());

    canvas_8.listener.on_mouse = |el, state, event| {
        if event.mouse == Mouse::Down || event.mouse == Mouse::Drag {
            if state.size == 8 {
                if event.modifiers.contains(&KeyMod::Ctrl) {
                    // Handle ctrl-click for color picking
                    let row = event.y.saturating_sub(el.pos.y.get()) as usize;
                    let col = event.x.saturating_sub(el.pos.x.get()) as usize / 2;
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
                el.look.set(look);
                crate::ui::draw_relative(el, X, Y, state);
            }
        }
    };
    canvas_8.listener.on_state = |el, state| {
        if state.size == 8 {
            let look = canvas_look_from_data(8, &state.canvas8_data);
            el.look.set(look);

            crate::ui::draw_relative(el, X, Y, state);
        }
    };

    canvas_8
}
