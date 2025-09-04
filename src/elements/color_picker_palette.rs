use crate::tui_engine::*;
use crate::AppState;

static X: u16 = 23;
static Y: u16 = 21;

pub fn build<'a>() -> Element<'a, AppState> {
    let mut color_picker_palette: Element<AppState> = Element::new(X, Y, {
        let row: Vec<String> = (0..32)
            .map(|index| {
                if index % 4 == 1 || index % 4 == 2 {
                    ":".to_string()
                } else {
                    " ".to_string()
                }
            })
            .collect();
        Look::from(vec![row])
    });

    color_picker_palette.on_move = Some(Box::new(|el, state, event| {
        if mouse_over(el, event) {
            let col_rel = event.x.unwrap().saturating_sub(el.x.get()) as usize;
            let selected = if col_rel % 4 == 1 || col_rel % 4 == 2 {
                col_rel / 4
            } else {
                state.palette_index
            };

            if selected < state.palette_colors.len() {
                state.candidate = state.palette_colors[selected];
                state.picker_mode = true;
            }
        }
    }));
    color_picker_palette.on_click = Some(Box::new(|el, state, event| {
        if mouse_over(el, event) {
            let col_rel = event.x.unwrap().saturating_sub(el.x.get()) as usize;
            let selected = if col_rel % 4 == 1 || col_rel % 4 == 2 {
                col_rel / 4
            } else {
                state.palette_index
            };

            if selected < state.palette_colors.len() {
                state.paintbrush = state.palette_colors[selected];
                state.palette_index = selected;
            }
        }
    }));
    color_picker_palette.on_state = Some(Box::new(|el, state| {
        let pl = state.palette_index;
        let pll = &state.palette_colors;

        let mut look = el.look.cells().to_vec();

        for row in look.iter_mut() {
            for (col_i, col) in row.iter_mut().enumerate() {
                if col_i % 4 == 1 || col_i % 4 == 2 {
                    let palette_idx = (col_i / 4).min(pll.len().saturating_sub(1));
                    let coloring = pll[palette_idx];
                    let active = col_i == pl * 4 + 1 || col_i == pl * 4 + 2;

                    *col = if let Some(c) = coloring {
                        terminal_style::format::background(c, if active { "+" } else { " " })
                            .unwrap()
                    } else {
                        if active {
                            "+".to_string()
                        } else {
                            ":".to_string()
                        }
                    };
                } else {
                    *col = " ".to_string();
                }
            }
        }

        el.look.update(look);

        crate::elements::draw_relative(el, X, Y, state);
    }));

    color_picker_palette
}
