use crate::tui_engine::*;
use crate::AppState;

const X: u16 = 61;
const Y: u16 = 9;

pub fn build<'a>() -> Element<'a, AppState> {
    let mut color_selected: Element<AppState> = Element::new(
X, Y,
        Look::from(vec![vec![" ".to_string(); 15]; 2]),
    );

    color_selected.on_state = Some(Box::new(|el, state| {
        let rows: Vec<Vec<String>> = (0..2)
            .map(|_| {
                (0..15)
                    .map(|_| {
                        state
                            .paintbrush
                            .map(|pb| terminal_style::format::background(pb, " ").unwrap())
                            .unwrap_or_else(|| " ".to_string())
                    })
                    .collect()
            })
            .collect();
        el.look.update(rows);

        crate::elements::draw_relative(el, X, Y, state);
    }));

    color_selected
}