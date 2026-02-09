use crate::AppState;
use little_tui::*;

static X: isize = 61;
static Y: isize = 9;

pub fn build() -> Element<AppState> {
    let color_selected: Element<AppState> = Element::new();
    color_selected
        .x(X)
        .y(Y)
        .look(Look::from((15, 2, ' ')))
        .on_state(|el, state| {
            el.background(state.paintbrush);
            el.decorate();
            crate::ui::draw_relative(el, X, Y, state);
        });

    color_selected
}
