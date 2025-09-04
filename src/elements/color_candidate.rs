use crate::tui_engine::*;
use crate::AppState;

static X: u16 = 61;
static Y: u16 = 11;

pub fn build<'a>() -> Element<'a, AppState> {
    let mut color_candidate: Element<AppState> =
        Element::new(X, Y, Look::from(vec![vec![" ".to_string(); 15]; 2]));

    color_candidate.on_state = Some(Box::new(|el, state| {
        let look = Look::from(vec![vec![" ".to_string(); 15]; 2]);

        let color_source = if state.picker_mode {
            state.candidate
        } else {
            state.paintbrush
        };

        if let Some(cs) = color_source {
            let styled = terminal_style::format::background(cs, look).unwrap();
            el.look.update(styled);
        }

        crate::elements::draw_relative(el, X, Y, state);
    }));

    color_candidate
}
