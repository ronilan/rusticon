use terminal_style::{
    color::{ansi8_to_hex, ansi8_to_rgb, rgb_to_ansi8},
    format::{background, inverse, underline},
};

use crate::tui_engine::{
    clear_below, columns, draw, go_to, mouse_over, rows, Element, Elements, Look,
};

use crate::AppState;

const APP_WIDTH: u16 = 80;
const APP_HEIGHT: u16 = 24;

fn dynamic_x(relative_x: u16) -> u16 {
    let base = columns().saturating_sub(APP_WIDTH) / 2;
    base + relative_x
}

fn dynamic_y(relative_y: u16) -> u16 {
    let base = rows().saturating_sub(APP_HEIGHT) / 2;
    base + relative_y
}

fn draw_if_fits<S>(el: &Element<S>) {
    if columns() >= APP_WIDTH && rows() >= APP_HEIGHT {
        draw(el);
    }
}

// Palette helpers
fn ideal_palette_index(curr: usize, palette: &Vec<Option<u8>>) -> usize {
    match palette.iter().position(|c| c.is_none()) {
        Some(idx) => idx,                   // first empty slot
        None => (curr + 1) % palette.len(), // wrap around if full
    }
}

fn set_palette_in_state(state: &mut AppState, value: Option<u8>) {
    let curr = state.palette_index;

    // Update the palette slot at `curr`
    if curr < state.palette_colors.len() {
        state.palette_colors[curr] = value;
    }

    // Move to the next ideal index
    state.palette_index = ideal_palette_index(curr, &state.palette_colors);
}

/// Canvas helpers
fn canvas_look_from_data(size: usize, data: &[Option<u8>]) -> Look {
    let rows: Vec<Vec<String>> = (0..size)
        .map(|row| {
            (0..(size * 2))
                .map(|col| {
                    let half_col = col / 2;
                    match data[row * size + half_col] {
                        Some(color) => background(color, " ").unwrap(),
                        None => ":".to_string(),
                    }
                })
                .collect()
        })
        .collect();
    Look::from(rows)
}

/// Simple 4-way flood fill
fn flood_fill(
    data: &mut Vec<Option<u8>>,
    size: usize,
    row: usize,
    col: usize,
    target: Option<u8>,
    replacement: Option<u8>,
) {
    if row >= size || col >= size {
        return;
    }
    let idx = row * size + col;
    if data[idx] != target {
        return;
    }
    data[idx] = replacement;

    // Up
    if row > 0 {
        flood_fill(data, size, row - 1, col, target, replacement);
    }
    // Down
    if row + 1 < size {
        flood_fill(data, size, row + 1, col, target, replacement);
    }
    // Left
    if col > 0 {
        flood_fill(data, size, row, col - 1, target, replacement);
    }
    // Right
    if col + 1 < size {
        flood_fill(data, size, row, col + 1, target, replacement);
    }
}

fn canvas_data_from_click(
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

// Main
pub fn build_elements<'a>() -> Elements<'a, AppState> {
    let elements: Elements<AppState> = Elements::new();

    // ---------------- Screen ---------------- //
    // utility element with no visible look.
    // clears screen on resize, "mouse out" of pickers.
    let mut screen: Element<AppState> = Element::new(0, 0, Look::new());

    screen.on_loop = Some(Box::new(|_el, state, _event| {
        let x = columns().saturating_sub(APP_WIDTH) / 2;
        let y = rows().saturating_sub(APP_HEIGHT) / 2;

        if x != state.app_x || y != state.app_y {
            go_to(0, 1);
            clear_below();

            state.app_x = x;
            state.app_y = y;
        }
    }));
    screen.on_move = Some(Box::new(|_el, state, _event| {
        state.picker_mode = false;
    }));

    elements.push(screen);

    // ---------------- Size Modal ---------------- //
    let mut centered_modal: Element<AppState> =
        Element::new(0, 0, Look::from(vec![vec!["".to_string()]]));

    centered_modal.on_state = Some(Box::new(|el, _state| {
        let terminal_too_small = columns() < APP_WIDTH || rows() < APP_HEIGHT;
        let mut look_rows = Vec::new();

        if terminal_too_small {
            let rows = rows() as usize;
            let columns = columns() as usize;

            for row_idx in 0..rows {
                let mut row = vec![" ".to_string(); columns];

                if row_idx == rows / 2 {
                    let mut point = columns / 2;
                    let msg_offset = 12;
                    point = point.saturating_sub(msg_offset);

                    let msg = "Enlarge Terminal Window";
                    for (i, ch) in msg.chars().enumerate() {
                        if point + i < row.len() {
                            row[point + i] = ch.to_string();
                        }
                    }
                }

                look_rows.push(row);
            }

            el.look.update(look_rows);
            draw(el);
        } else {
            el.look.update("");
            draw(el);
        }
    }));

    elements.push(centered_modal);

    // ---------------- Title Bar ---------------- //
    let mut title_bar: Element<AppState> = Element::new(0, 0, Look::new());

    title_bar.on_state = Some(Box::new(move |el, state| {
        let cols = crossterm::terminal::size().unwrap().0 as usize;

        let mut line = " ".repeat(cols);
        let text = format!(
            "Rusticon: {} {}x{}",
            state.file_path, state.size, state.size
        );
        line.replace_range(0..text.len().min(cols), &text);

        el.look.update(vec![vec![inverse(&line)]]);
        draw(el);
    }));

    elements.push(title_bar);

    // ---------------- Canvas 16 x 16 ---------------- //
    let mut canvas16: Element<AppState> = Element::new(
        dynamic_x(23),
        dynamic_y(3),
        Look::from(""), //The actual look is set in on_state. Default is 8x8
    );

    canvas16.on_click = Some(Box::new(|el, state, event| {
        if state.size == 16 {
            state.mouse_x = event.x.unwrap();
            state.mouse_y = event.y.unwrap();
            if mouse_over(
                el.x.get(),
                el.y.get(),
                &el.look,
                state.mouse_x,
                state.mouse_y,
            ) {
                if event.modifiers.contains(&"ctrl".to_string()) {
                    // Handle ctrl-click for color picking
                    let row = state.mouse_y.saturating_sub(el.y.get()) as usize;
                    let col = state.mouse_x.saturating_sub(el.x.get()) as usize / 2;
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
                        state.mouse_x,
                        state.mouse_y,
                        event.modifiers.contains(&"shift".to_string()),
                    );
                }
            }

            let look = canvas_look_from_data(16, &state.canvas16_data);
            el.look.update(look);
            draw_if_fits(el);
        }
    }));
    canvas16.on_state = Some(Box::new(|el, state| {
        if state.size == 16 {
            let look = canvas_look_from_data(16, &state.canvas16_data);
            el.look.update(look);

            el.x.set(state.app_x + 23);
            el.y.set(state.app_y + 3);
            draw_if_fits(el);
        }
    }));

    elements.push(canvas16);

    // ---------------- Canvas 8 x 8 ---------------- //
    let mut canvas8: Element<AppState> = Element::new(
        dynamic_x(31),
        dynamic_y(7),
        Look::from(""), //The actual look is set in on_state. Default is 8x8.
    );

    canvas8.on_click = Some(Box::new(|el, state, event| {
        if state.size == 8 {
            state.mouse_x = event.x.unwrap();
            state.mouse_y = event.y.unwrap();

            if mouse_over(
                el.x.get(),
                el.y.get(),
                &el.look,
                state.mouse_x,
                state.mouse_y,
            ) {
                if event.modifiers.contains(&"ctrl".to_string()) {
                    // Handle ctrl-click for color picking
                    let row = state.mouse_y.saturating_sub(el.y.get()) as usize;
                    let col = state.mouse_x.saturating_sub(el.x.get()) as usize / 2;
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
                        state.mouse_x,
                        state.mouse_y,
                        event.modifiers.contains(&"shift".to_string()),
                    );
                }
            }

            let look = canvas_look_from_data(8, &state.canvas8_data);
            el.look.update(look);
            draw_if_fits(el);
        }
    }));
    canvas8.on_state = Some(Box::new(|el, state| {
        if state.size == 8 {
            let look = canvas_look_from_data(8, &state.canvas8_data);
            el.look.update(look);

            el.x.set(state.app_x + 31);
            el.y.set(state.app_y + 7);
            draw_if_fits(el);
        }
    }));

    elements.push(canvas8);

    // ---------------- Color Picker 216 ---------------- //
    let mut color_picker: Element<AppState> = Element::new(
        dynamic_x(3),
        dynamic_y(2),
        Look::from(
            (0..18)
                .map(|row| {
                    (0..12)
                        .map(|col| {
                            // original crumb formula: (row * 12) + (col * 16)
                            let code = (row * 12 + col + 16).min(231) as u8;
                            background(code, " ").unwrap()
                        })
                        .collect::<Vec<String>>()
                })
                .collect::<Vec<Vec<String>>>(),
        ),
    );
    color_picker.on_move = Some(Box::new(|el, state, event| {
        state.mouse_x = event.x.unwrap();
        state.mouse_y = event.y.unwrap();
        if mouse_over(
            el.x.get(),
            el.y.get(),
            &el.look,
            state.mouse_x,
            state.mouse_y,
        ) {
            let row = state.mouse_y.saturating_sub(el.y.get()) as u8;
            let col = state.mouse_x.saturating_sub(el.x.get()) as u8;
            let code = row * 12 + col + 16;
            state.candidate = Some(code);
            state.picker_mode = true;
        }
    }));
    color_picker.on_click = Some(Box::new(|el, state, event| {
        state.mouse_x = event.x.unwrap();
        state.mouse_y = event.y.unwrap();
        if mouse_over(
            el.x.get(),
            el.y.get(),
            &el.look,
            state.mouse_x,
            state.mouse_y,
        ) {
            let row = state.mouse_y.saturating_sub(el.y.get()) as u8;
            let col = state.mouse_x.saturating_sub(el.x.get()) as u8;
            let code = row * 12 + col + 16;
            state.paintbrush = Some(code);
            state.candidate = Some(code);
            set_palette_in_state(state, state.candidate);
        }
    }));
    color_picker.on_state = Some(Box::new(|el, state| {
        el.x.set(state.app_x + 3);
        el.y.set(state.app_y + 2);

        draw_if_fits(el);
    }));

    elements.push(color_picker);

    // ---------------- Gray Picker ---------------- //
    let mut gray_picker: Element<AppState> = Element::new(
        dynamic_x(16),
        dynamic_y(2),
        Look::from(
            (0..12)
                .map(|row| {
                    (0..2)
                        .map(|col| {
                            let code: u8 = (row * 2 + col + 232).try_into().unwrap();
                            background(code, " ").unwrap()
                        })
                        .collect::<Vec<String>>() // a single row
                })
                .collect::<Vec<Vec<String>>>(), // all rows
        ),
    );
    gray_picker.on_move = Some(Box::new(|el, state, event| {
        state.mouse_x = event.x.unwrap();
        state.mouse_y = event.y.unwrap();
        if mouse_over(
            el.x.get(),
            el.y.get(),
            &el.look,
            state.mouse_x,
            state.mouse_y,
        ) {
            let row = state.mouse_y.saturating_sub(el.y.get()) as u8;
            let col = state.mouse_x.saturating_sub(el.x.get()) as u8;
            let code = (row * 2 + col + 232) as u8;
            state.candidate = Some(code);
            state.picker_mode = true;
        }
    }));
    gray_picker.on_click = Some(Box::new(|el, state, event| {
        state.mouse_x = event.x.unwrap();
        state.mouse_y = event.y.unwrap();
        if mouse_over(
            el.x.get(),
            el.y.get(),
            &el.look,
            state.mouse_x,
            state.mouse_y,
        ) {
            let row = state.mouse_y.saturating_sub(el.y.get());
            let col = state.mouse_x.saturating_sub(el.x.get());
            let code = (row * 2 + col + 232) as u8;
            state.paintbrush = Some(code);
            state.candidate = Some(code);
            set_palette_in_state(state, state.candidate);
        }
    }));
    gray_picker.on_state = Some(Box::new(|el, state| {
        el.x.set(state.app_x + 16);
        el.y.set(state.app_y + 2);

        draw_if_fits(el);
    }));

    elements.push(gray_picker);

    // ---------------- Base Colors (16) Picker ---------------- //
    let mut sixteen_picker: Element<AppState> = Element::new(
        dynamic_x(1),
        dynamic_y(2),
        Look::from(
            (0..16)
                .map(|row| {
                    (0..1)
                        .map(|_col| {
                            let ansi_code: u8 = rgb_to_ansi8(ansi8_to_rgb(row)).try_into().unwrap();
                            background(ansi_code, " ").unwrap()
                        })
                        .collect::<Vec<String>>()
                })
                .collect::<Vec<Vec<String>>>(),
        ),
    );

    sixteen_picker.on_move = Some(Box::new(|el, state, event| {
        state.mouse_x = event.x.unwrap();
        state.mouse_y = event.y.unwrap();
        if mouse_over(
            el.x.get(),
            el.y.get(),
            &el.look,
            state.mouse_x,
            state.mouse_y,
        ) {
            let row = state.mouse_y.saturating_sub(el.y.get()) as u8;
            let code = rgb_to_ansi8(ansi8_to_rgb(row));
            state.candidate = Some(code);
            state.picker_mode = true;
        }
    }));
    sixteen_picker.on_click = Some(Box::new(|el, state, event| {
        state.mouse_x = event.x.unwrap();
        state.mouse_y = event.y.unwrap();
        if mouse_over(
            el.x.get(),
            el.y.get(),
            &el.look,
            state.mouse_x,
            state.mouse_y,
        ) {
            let row = state.mouse_y.saturating_sub(el.y.get()) as u8;
            let code = rgb_to_ansi8(ansi8_to_rgb(row));
            state.paintbrush = Some(code);
            state.candidate = Some(code);
            set_palette_in_state(state, state.candidate);
        }
    }));
    sixteen_picker.on_state = Some(Box::new(|el, state| {
        el.x.set(state.app_x + 1);
        el.y.set(state.app_y + 2);

        draw_if_fits(el);
    }));

    elements.push(sixteen_picker);

    // ---------------- Transparent Picker ---------------- //
    let mut empty_picker: Element<AppState> = Element::new(
        dynamic_x(16),
        dynamic_y(18),
        Look::from(vec![
            vec![":".to_string(), ":".to_string()],
            vec![":".to_string(), ":".to_string()],
        ]),
    );

    empty_picker.on_move = Some(Box::new(|el, state, event| {
        state.mouse_x = event.x.unwrap();
        state.mouse_y = event.y.unwrap();
        if mouse_over(
            el.x.get(),
            el.y.get(),
            &el.look,
            state.mouse_x,
            state.mouse_y,
        ) {
            state.candidate = None;
            state.picker_mode = true;
        }
    }));
    empty_picker.on_click = Some(Box::new(|el, state, event| {
        state.mouse_x = event.x.unwrap();
        state.mouse_y = event.y.unwrap();
        if mouse_over(
            el.x.get(),
            el.y.get(),
            &el.look,
            state.mouse_x,
            state.mouse_y,
        ) {
            state.paintbrush = None;
            state.candidate = None;
            set_palette_in_state(state, state.candidate);
        }
    }));
    empty_picker.on_state = Some(Box::new(|el, state| {
        el.x.set(state.app_x + 16);
        el.y.set(state.app_y + 18);

        draw_if_fits(el);
    }));

    elements.push(empty_picker);

    // ---------------- Palette Picker ---------------- //
    let mut palette_picker: Element<AppState> = Element::new(dynamic_x(23), dynamic_y(21), {
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

    palette_picker.on_move = Some(Box::new(|el, state, event| {
        if let (Some(mouse_x), Some(mouse_y)) = (event.x, event.y) {
            state.mouse_x = mouse_x;
            state.mouse_y = mouse_y;

            if mouse_over(el.x.get(), el.y.get(), &el.look, mouse_x, mouse_y) {
                let col_rel = mouse_x.saturating_sub(el.x.get()) as usize;
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
        }
    }));
    palette_picker.on_click = Some(Box::new(|el, state, event| {
        state.mouse_x = event.x.unwrap();
        state.mouse_y = event.y.unwrap();

        if mouse_over(
            el.x.get(),
            el.y.get(),
            &el.look,
            state.mouse_x,
            state.mouse_y,
        ) {
            let col_rel = state.mouse_x.saturating_sub(el.x.get()) as usize;

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
    palette_picker.on_state = Some(Box::new(|el, state| {
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
                        background(c, if active { "+" } else { " " }).unwrap()
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
        el.x.set(state.app_x + 23);
        el.y.set(state.app_y + 21);
        draw_if_fits(el);
    }));

    elements.push(palette_picker);

    // ---------------- Color Selected Label ---------------- //
    let mut color_selected_label: Element<AppState> = Element::new(
        dynamic_x(62),
        dynamic_y(8),
        Look::from(vec![vec!["".to_string()]]),
    );

    color_selected_label.on_state = Some(Box::new(|el, state| {
        let text = match state.paintbrush {
            Some(c) => format!("{:<3}   {}", c, ansi8_to_hex(c)),
            None => format!("{:<13}", ":transparent:"),
        };
        el.look.update(text);
        el.x.set(state.app_x + 62);
        el.y.set(state.app_y + 8);
        draw_if_fits(el);
    }));

    elements.push(color_selected_label);

    // ---------------- Color Selected Box ---------------- //
    let mut color_selected: Element<AppState> = Element::new(
        dynamic_x(61),
        dynamic_y(9),
        Look::from(vec![vec![" ".to_string(); 15]; 2]),
    );

    color_selected.on_state = Some(Box::new(|el, state| {
        let rows: Vec<Vec<String>> = (0..2)
            .map(|_| {
                (0..15)
                    .map(|_| {
                        state
                            .paintbrush
                            .map(|pb| background(pb, " ").unwrap())
                            .unwrap_or_else(|| " ".to_string())
                    })
                    .collect()
            })
            .collect();
        el.look.update(rows);
        el.x.set(state.app_x + 61);
        el.y.set(state.app_y + 9);
        draw_if_fits(el);
    }));

    elements.push(color_selected);

    // ---------------- Color Candidate Label ---------------- //
    let mut color_candidate_label: Element<AppState> = Element::new(
        dynamic_x(62),
        dynamic_y(13),
        Look::from(vec![vec!["".to_string()]]),
    );

    color_candidate_label.on_state = Some(Box::new(|el, state| {
        let text = if state.picker_mode {
            match state.candidate {
                Some(c) => format!("{:<3}   {}", c, ansi8_to_hex(c)),
                None => format!("{:<13}", ":transparent:"),
            }
        } else {
            "             ".to_string()
        };

        el.look.update(text);
        el.x.set(state.app_x + 62);
        el.y.set(state.app_y + 13);
        draw_if_fits(el);
    }));

    elements.push(color_candidate_label);

    // ---------------- Color Candidate Box ---------------- //
    let mut color_candidate: Element<AppState> = Element::new(
        dynamic_x(61),
        dynamic_y(11),
        Look::from(vec![vec![" ".to_string(); 15]; 2]),
    );

    color_candidate.on_state = Some(Box::new(|el, state| {
        let rows: Vec<Vec<String>> = (0..2)
            .map(|_| {
                (0..15)
                    .map(|_| {
                        let color_source = if state.picker_mode {
                            state.candidate
                        } else {
                            state.paintbrush
                        };
                        color_source
                            .map(|c| background(c, " ").unwrap())
                            .unwrap_or_else(|| " ".to_string())
                    })
                    .collect()
            })
            .collect();
        el.look.update(rows);
        el.x.set(state.app_x + 61);
        el.y.set(state.app_y + 11);
        draw_if_fits(el);
    }));

    elements.push(color_candidate);

    // ---------------- Label: "New:" ---------------- //
    let mut label_size: Element<AppState> =
        Element::new(dynamic_x(59), dynamic_y(2), Look::from("New:"));

    label_size.on_state = Some(Box::new(|el, state| {
        el.x.set(state.app_x + 59);
        el.y.set(state.app_y + 2);
        draw_if_fits(el);
    }));

    elements.push(label_size);

    // ---------------- Button: 8x8 ---------------- //
    let mut button_size_8: Element<AppState> =
        Element::new(dynamic_x(67), dynamic_y(2), underline(Look::from("8x8")));

    button_size_8.on_click = Some(Box::new(|el, state, event| {
        if mouse_over(
            el.x.get(),
            el.y.get(),
            &el.look,
            event.x.unwrap(),
            event.y.unwrap(),
        ) {
            state.size = 8;
            state.canvas8_data = vec![None; 64];

            // erase the 16x16 area
            let eraser: Element<AppState> = Element::new(
                state.app_x + 23,
                state.app_y + 3,
                Look::from(vec![vec![" ".to_string(); 32]; 16]),
            );
            draw_if_fits(&eraser);
        }
    }));
    button_size_8.on_state = Some(Box::new(|el, state| {
        el.x.set(state.app_x + 67);
        el.y.set(state.app_y + 2);
        draw_if_fits(el);
    }));

    elements.push(button_size_8);

    // ---------------- Button: 16x16 ---------------- //
    let mut button_size_16: Element<AppState> =
        Element::new(dynamic_x(74), dynamic_y(2), underline(Look::from("16x16")));

    button_size_16.on_click = Some(Box::new(|el, state, event| {
        if mouse_over(
            el.x.get(),
            el.y.get(),
            &el.look,
            event.x.unwrap(),
            event.y.unwrap(),
        ) {
            state.size = 16;
            state.canvas16_data = vec![None; 256];
        }
    }));
    button_size_16.on_state = Some(Box::new(|el, state| {
        el.x.set(state.app_x + 74);
        el.y.set(state.app_y + 2);
        draw_if_fits(el);
    }));

    elements.push(button_size_16);

    // ---------------- Label: "End:" ---------------- //
    let mut label_finish: Element<AppState> =
        Element::new(dynamic_x(59), dynamic_y(19), Look::from("End:"));

    label_finish.on_state = Some(Box::new(|el, state| {
        el.x.set(state.app_x + 59);
        el.y.set(state.app_y + 19);
        draw_if_fits(el);
    }));

    elements.push(label_finish);

    // ---------------- Button: "Exit" ---------------- //
    let mut button_exit: Element<AppState> =
        Element::new(dynamic_x(67), dynamic_y(19), underline(Look::from("Exit")));
    button_exit.on_keypress = Some(Box::new(|_el, state, event| {
        if event.key == Some("c".to_string()) && event.modifiers.contains(&"ctrl".to_string()) {
            state.exit_flag = true;
        }
    }));
    button_exit.on_click = Some(Box::new(|el, state, event| {
        if mouse_over(
            el.x.get(),
            el.y.get(),
            &el.look,
            event.x.unwrap(),
            event.y.unwrap(),
        ) {
            state.exit_flag = true;
        }
    }));
    button_exit.on_state = Some(Box::new(|el, state| {
        el.x.set(state.app_x + 67);
        el.y.set(state.app_y + 19);
        draw_if_fits(el);
    }));

    elements.push(button_exit);

    // ---------------- Button: "Save" ---------------- //
    let mut button_save: Element<AppState> =
        Element::new(dynamic_x(75), dynamic_y(19), underline(Look::from("Save")));

    button_save.on_loop = Some(Box::new(|_el, state, _event| {
        if state.save_flag {
            // wait till next loop to exit
            state.exit_flag = true;
        }
    }));
    button_save.on_click = Some(Box::new(|el, state, event| {
        if let (Some(mx), Some(my)) = (event.x, event.y) {
            if mouse_over(el.x.get(), el.y.get(), &el.look, mx, my) {
                state.save_flag = true;
            }
        }
    }));
    button_save.on_state = Some(Box::new(|el, state| {
        el.x.set(state.app_x + 75);
        el.y.set(state.app_y + 19);
        draw_if_fits(el);
    }));

    elements.push(button_save);

    elements
}
