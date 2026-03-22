use little_tui::*;

/// Draws a full-screen message centered in the terminal
pub fn draw_message(msg: &str, color_code: u8) {
    let term_cols = Terminal::columns() as usize;
    let term_rows = Terminal::rows() as usize;

    // Split message into lines
    let message_lines: Vec<String> = msg.lines().map(|s| s.to_string()).collect();
    let msg_height = message_lines.len();

    // Full-screen 2D vector of spaces
    let mut screen: Vec<Vec<char>> = vec![vec![' '; term_cols]; term_rows];

    let start_y = term_rows.saturating_sub(msg_height) / 2;

    for (i, line) in message_lines.iter().enumerate() {
        let start_x = term_cols.saturating_sub(line.len()) / 2;
        for (j, ch) in line.chars().enumerate() {
            if start_x + j < term_cols {
                screen[start_y + i][start_x + j] = ch;
            }
        }
    }

    let look = Look::from(screen);

    // Draw element
    let el: Element<()> = Element::new();
    el.look(look)
        .bold(true)
        .color(Some(Color::Ansi(color_code)));
    decorate(&el);
    draw(&el);
}
