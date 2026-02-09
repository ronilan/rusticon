use crate::ui::utils::*;
use crate::{ui::reposition, AppState};
use little_tui::*;

static X: isize = 16;
static Y: isize = 2;

pub fn build() -> Element<AppState> {
    let color_picker_gray: Element<AppState> = Element::new();
    color_picker_gray
        .x(X)
        .y(Y)
        .look(Look::from(
            (0..12)
                .map(|row| {
                    (0..2)
                        .map(|col| {
                            let ansi_code: u8 = (row * 2 + col + 232).try_into().unwrap();
                            let decor =
                                Decor::new(false, false, false, false, None, Some(ansi_code));
                            Block::new(' ', decor)
                        })
                        .collect::<Vec<Block>>()
                })
                .collect::<Vec<Vec<Block>>>(),
        ))
        .on_mouse(|el, state, event| {
            if event.mouse == Mouse::Move || event.mouse == Mouse::Click {
                let row = event.y.saturating_sub(el.visual.y.get()) as u8;
                let col = event.x.saturating_sub(el.visual.x.get()) as u8;

                let ansi_code = row * 2 + col + 232;
                state.candidate = Some(ansi_code);
                state.picker_mode = true;

                if event.mouse == Mouse::Move {
                    state.picker_mode = true;
                }
                if event.mouse == Mouse::Click {
                    state.paintbrush = Some(ansi_code);
                    set_palette_in_state(state, state.candidate);
                }
            }
        })
        .on_state(|el, state| {
            reposition(el, X, Y, state);
            el.draw();
        });

    color_picker_gray
}
