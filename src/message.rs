use terminal_style::format::{bold, color};

use crate::tui_engine::{columns, draw, rows, Element, Look};

/// Draws a full-screen message centered in the terminal
pub fn draw_message(msg: &str, color_code: u8) {
    let term_cols = columns() as usize;
    let term_rows = rows() as usize;

    // Split message into lines
    let message_lines: Vec<String> = msg.lines().map(|s| s.to_string()).collect();
    let msg_height = message_lines.len();

    // Full-screen 2D vector of spaces
    let mut screen: Vec<Vec<String>> = vec![vec![" ".to_string(); term_cols]; term_rows];

    let start_y = term_rows.saturating_sub(msg_height) / 2;

    for (i, line) in message_lines.iter().enumerate() {
        let start_x = term_cols.saturating_sub(line.len()) / 2;
        for (j, ch) in line.chars().enumerate() {
            if start_x + j < term_cols {
                screen[start_y + i][start_x + j] = ch.to_string();
            }
        }
    }

    // Create Look from screen and wrap in color
    let look = bold(color(color_code, Look::from(screen)).unwrap());

    // Draw element
    let mut el: Element<'_, ()> = Element::new(0, 0, look);
    draw(&mut el);
}
