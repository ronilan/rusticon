use crate::tui_engine::*;
use crate::AppState;

// Palette helpers
pub(crate) fn ideal_palette_index(curr: usize, palette: &Vec<Option<u8>>) -> usize {
    match palette.iter().position(|c| c.is_none()) {
        Some(idx) => idx, // first empty slot
        None => curr,     // stay in place if full
    }
}

pub(crate) fn set_palette_in_state(state: &mut AppState, value: Option<u8>) {
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
    let rows: Vec<Vec<String>> = (0..size)
        .map(|row| {
            (0..(size * 2))
                .map(|col| {
                    let half_col = col / 2;
                    match data[row * size + half_col] {
                        Some(color) => terminal_style::format::background(color, " ").unwrap(),
                        None => ":".to_string(),
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
    el: &Element<AppState>,
    size: usize,
    data: &mut Vec<Option<u8>>,
    paintbrush: Option<u8>,
    mouse_x: u16,
    mouse_y: u16,
    fill: bool,
) {
    let row = mouse_y.saturating_sub(el.y.get()) as usize;
    let col = mouse_x.saturating_sub(el.x.get()) as usize / 2;
    if row < size && col < size {
        if fill {
            // Flood fill starting at (row, col)
            let target = data[row * size + col];
            if target != paintbrush {
                flood_fill(data, size, row, col, target, paintbrush);
            }
        } else {
            // Single cell paint
            data[row * size + col] = paintbrush;
        }
    }
}
