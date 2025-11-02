use crate::AppState;
use little_tui::*;

static X: i16 = 61;
static Y: i16 = 11;

pub fn build() -> Element<AppState> {
    let mut color_candidate: Element<AppState> =
        Element::new(Pos::new(X, Y), Look::from((15, 2, ' ')));

    color_candidate.listener.on_state = |el, state| {
        let look = Look::from((15, 2, ' '));

        let color_source = if state.picker_mode {
            state.candidate
        } else {
            state.paintbrush
        };

        if let Some(cs) = color_source {
            let styled = terminal_style::format::background(cs, look).unwrap();
            el.look.update(styled);
        } else {
            el.look.update(look);
        }

        crate::ui::draw_relative(el, X, Y, state);
    };

    color_candidate
}
