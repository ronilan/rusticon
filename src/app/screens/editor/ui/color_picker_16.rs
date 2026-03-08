use super::utils::*;
use crate::State;
use little_tui::*;

static X: isize = 1;
static Y: isize = 2;

pub fn build() -> Element<State> {
    let color_picker_16: Element<State> = Element::new();
    color_picker_16
        .x(X)
        .y(Y)
        .look(Look::from(
            (0..16)
                .map(|row| {
                    (0..1)
                        .map(|_col| {
                            let ansi_code: u8 = Colors::rgb_to_ansi8(Colors::ansi8_to_rgb(row));
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
                let ansi_code: u8 = Colors::rgb_to_ansi8(Colors::ansi8_to_rgb(row));
                state.editor.candidate = Some(ansi_code);

                if event.mouse == Mouse::Move {
                    state.editor.picker_mode = true;
                }
                if event.mouse == Mouse::Click {
                    state.editor.paintbrush = Some(ansi_code);
                    set_palette_in_state(state, state.editor.candidate);
                }
            }
        })
        .on_state(|el, _state| {
            el.draw();
        });

    color_picker_16
}
