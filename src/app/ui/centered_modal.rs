use crate::ui::{APP_HEIGHT, APP_WIDTH};
use crate::State;
use incredible::*;

pub fn build() -> Element<State> {
    let centered_modal: Element<State> = Element::new();

    centered_modal.on_state(|el, _state| {
        let terminal_too_small = Platform::columns() < APP_WIDTH || Platform::rows() < APP_HEIGHT;
        let mut look_rows = Vec::new();

        if terminal_too_small {
            let rows = Platform::rows() as usize;
            let columns = Platform::columns() as usize;

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
            el.draw();
        } else {
            el.look(Look::new());
            el.draw();
        }
    });

    centered_modal
}
