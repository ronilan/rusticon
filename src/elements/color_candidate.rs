use crate::tui_engine::*;
use crate::AppState;

const X: u16 = 61;
const Y: u16 = 11;

pub fn build<'a>() -> Element<'a, AppState> {
    let mut color_candidate: Element<AppState> = Element::new(
X, Y,
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
                            .map(|c| terminal_style::format::background(c, " ").unwrap())
                            .unwrap_or_else(|| " ".to_string())
                    })
                    .collect()
            })
            .collect();
        el.look.update(rows);

        crate::elements::draw_relative(el, X, Y, state);
    }));

    color_candidate
}