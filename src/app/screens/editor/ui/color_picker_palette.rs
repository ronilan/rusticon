use crate::{ui::reposition, State};
use incredible::*;

static X: isize = 23;
static Y: isize = 21;

pub fn build<'a>() -> Element<State> {
    let color_picker_palette: Element<State> = Element::new();
    color_picker_palette
        .x(X)
        .y(Y)
        .look({
            let row: Vec<char> = (0..32)
                .map(|index| {
                    if index % 4 == 1 || index % 4 == 2 {
                        ':'
                    } else {
                        ' '
                    }
                })
                .collect();
            Look::from(vec![row])
        })
        .on_mouse(|el, state, event| {
            let col_rel = event.x.saturating_sub(el.visual.x.get()) as usize;
            let selected = if col_rel % 4 == 1 || col_rel % 4 == 2 {
                col_rel / 4
            } else {
                state.palette_index
            };

            if event.mouse == Mouse::Click {
                if selected < state.palette_colors.len() {
                    state.paintbrush = state.palette_colors[selected];
                    state.palette_index = selected;
                }
            }
            if event.mouse == Mouse::Move {
                if selected < state.palette_colors.len() {
                    state.candidate = state.palette_colors[selected];
                    state.picker_mode = true;
                }
            }
        })
        .on_state(|el, state| {
            let pl = state.palette_index;
            let pll = &state.palette_colors;

            let mut look = el.visual.look.blocks().to_vec();

            for row in look.iter_mut() {
                for (col_i, col) in row.iter_mut().enumerate() {
                    if col_i % 4 == 1 || col_i % 4 == 2 {
                        let palette_idx = (col_i / 4).min(pll.len().saturating_sub(1));
                        let coloring = pll[palette_idx];
                        let active = col_i == pl * 4 + 1 || col_i == pl * 4 + 2;

                        let mut decor = Decor::default();
                        if let Some(ansi_code) = coloring {
                            decor = Decor::new(
                                false,
                                false,
                                None,
                                None,
                                false,
                                false,
                                false,
                                None,
                                Some(Color::Ansi(ansi_code)),
                            );
                        }

                        let content = if active {
                            '+'
                        } else {
                            if coloring.is_none() {
                                ':'
                            } else {
                                ' '
                            }
                        };

                        *col = Block::new(content, decor);
                    } else {
                        *col = Block::new(' ', Decor::default());
                    }
                }
            }

            el.look(Look::from(look));

            reposition(el, X, Y, state);
            el.draw();
        });

    color_picker_palette
}
