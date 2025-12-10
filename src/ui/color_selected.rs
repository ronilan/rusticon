use crate::AppState;
use little_tui::*;

static X: i16 = 61;
static Y: i16 = 9;

pub fn build() -> Element<AppState> {
    let mut color_selected: Element<AppState> = Element::new();
    color_selected.x(X).y(Y).look(Look::from((15, 2, ' ')));

    color_selected.listener.on_state = |el, state| {
        el.background(state.paintbrush);
        decorate(el);
        crate::ui::draw_relative(el, X, Y, state);
    };

    color_selected
}
