use crate::AppState;
use little_tui::*;

static X: i16 = 61;
static Y: i16 = 9;

pub fn build() -> Element<AppState> {
    let mut color_selected: Element<AppState> =
        Element::new(Pos::new(X, Y), Look::from((15, 2, ' ')));

    color_selected.listener.on_state = |el, state| {
        let look = Look::from((15, 2, ' '));

        if let Some(pb) = state.paintbrush {
            let styled = terminal_style::format::background(pb, look).unwrap();
            el.look.update(styled);
        } else {
            el.look.update(look);
        }

        crate::ui::draw_relative(el, X, Y, state);
    };

    color_selected
}
