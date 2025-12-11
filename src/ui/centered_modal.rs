use crate::ui::{APP_HEIGHT, APP_WIDTH};
use crate::AppState;
use little_tui::*;

pub fn build() -> Element<AppState> {
    let mut centered_modal: Element<AppState> = Element::new();

    centered_modal.listener.on_state = |el, _state| {
        let terminal_too_small = Terminal::columns() < APP_WIDTH || Terminal::rows() < APP_HEIGHT;
        let mut look_rows = Vec::new();

        if terminal_too_small {
            let rows = Terminal::rows() as usize;
            let columns = Terminal::columns() as usize;

            for row_idx in 0..rows {
                let mut row = vec![' '; columns];

                if row_idx == rows / 2 {
                    let mut point = columns / 2;
                    let msg_offset = 12;
                    point = point.saturating_sub(msg_offset);

                    let msg = "Enlarge Terminal Window";
                    for (i, ch) in msg.chars().enumerate() {
                        if point + i < row.len() {
                            row[point + i] = ch;
                        }
                    }
                }

                look_rows.push(row);
            }

            el.look(Look::from(look_rows));
            draw(el);
        } else {
            el.look(Look::new());
            draw(el);
        }
    };

    centered_modal
}
