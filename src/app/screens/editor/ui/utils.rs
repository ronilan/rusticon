use crate::State;
use incredible::*;

// Palette helpers
pub(crate) fn ideal_palette_index(curr: usize, palette: &Vec<Option<u8>>) -> usize {
    match palette.iter().position(|c| c.is_none()) {
        Some(idx) => idx, // first empty slot
        None => curr,     // stay in place if full
    }
}

pub(crate) fn set_palette_in_state(state: &mut State, value: Option<u8>) {
    let curr = state.palette_index;

    // Update the palette slot at `curr`
    if curr < state.palette_colors.len() {
        state.palette_colors[curr] = value;
    }

    // Move to the next ideal index
    state.palette_index = ideal_palette_index(curr, &state.palette_colors);
}

/// Canvas helpers
pub(crate) fn canvas_look_from_data(size: usize, data: &[Option<u8>]) -> Look {
    let rows: Vec<Vec<Block>> = (0..size)
        .map(|row| {
            (0..(size * 2))
                .map(|col| {
                    let half_col = col / 2;
                    match data[row * size + half_col] {
                        Some(ansi_code) => {
                            let decor = Decor::new(
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
                            Block::new(' ', decor)
                        }
                        None => Block::new(':', Decor::default()),
                    }
                })
                .collect()
        })
        .collect();
    Look::from(rows)
}

/// Simple 4-way iterative flood fill
pub(crate) fn flood_fill(
    data: &mut Vec<Option<u8>>,
    size: usize,
    row: usize,
    col: usize,
    target: Option<u8>,
    replacement: Option<u8>,
) {
    let mut stack = vec![(row, col)];

    while let Some((r, c)) = stack.pop() {
        if r >= size || c >= size {
            continue;
        }

        let idx = r * size + c;
        if data[idx] != target {
            continue;
        }

        data[idx] = replacement;

        if r > 0 {
            stack.push((r - 1, c));
        } // Up
        if r + 1 < size {
            stack.push((r + 1, c));
        } // Down
        if c > 0 {
            stack.push((r, c - 1));
        } // Left
        if c + 1 < size {
            stack.push((r, c + 1));
        } // Right
    }
}

pub(crate) fn canvas_data_from_click(
    el: &Element<State>,
    size: usize,
    data: &mut Vec<Option<u8>>,
    paintbrush: Option<u8>,
    mouse_x: isize,
    mouse_y: isize,
    prev_color_on_canvas: &mut Option<u8>,
    fill: bool,
) {
    let row = mouse_y.saturating_sub(el.visual.y.get()) as usize;
    let col = mouse_x.saturating_sub(el.visual.x.get()) as usize / 2;
    if row < size && col < size {
        let idx = row * size + col;
        if fill {
            // Double-click: the second Down already painted this cell. Restore
            // the color from before the paint stroke, then flood-fill that region.
            data[idx] = *prev_color_on_canvas;
            let target = data[idx];
            if target != paintbrush {
                flood_fill(data, size, row, col, target, paintbrush);
            }
        } else {
            // Remember the cell color before paint so a following double-click
            // can undo the stroke and flood-fill. Skip when the cell is already
            // the paintbrush color so the second click of a double-click does
            // not overwrite the original color with the brush.
            let old = data[idx];
            if old != paintbrush {
                *prev_color_on_canvas = old;
            }
            data[idx] = paintbrush;
        }
    }
}
