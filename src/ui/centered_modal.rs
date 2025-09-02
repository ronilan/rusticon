use crate::tui_engine::*;
use crate::ui::{APP_HEIGHT, APP_WIDTH};
use crate::AppState;

pub fn build<'a>() -> Element<'a, AppState> {
    let mut centered_modal: Element<AppState> = Element::new(0, 0, Look::new());

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

    centered_modal
}
