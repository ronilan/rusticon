use super::utils::*;
use crate::core::model::State;
use little_tui::*;

static X: isize = 3;
static Y: isize = 2;

pub fn build() -> Element<State> {
    let color_picker_216: Element<State> = Element::new();
    color_picker_216
        .x(X)
        .y(Y)
        .look(Look::from(
            (0..18)
                .map(|row| {
                    (0..12)
                        .map(|col| {
                            // original crumb formula: (row * 12) + (col * 16)
                            let ansi_code = (row * 12 + col + 16).min(231) as u8;
                            let decor = Decor::new(
                                false,
                                false,
                                false,
                                false,
                                None,
                                Some(Color::Ansi(ansi_code)),
                            );
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
                let ansi_code = row * 12 + col + 16;
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

    color_picker_216
}
