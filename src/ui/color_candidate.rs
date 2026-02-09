use crate::{ui::reposition, AppState};
use little_tui::*;

static X: isize = 61;
static Y: isize = 11;

pub fn build() -> Element<AppState> {
    let color_candidate: Element<AppState> = Element::new();
    color_candidate
        .x(X)
        .y(Y)
        .look(Look::from((15, 2, ' ')))
        .on_state(|el, state| {
            let color_source = if state.picker_mode {
                state.candidate
            } else {
                state.paintbrush
            };

            el.background(color_source);

            reposition(el, X, Y, state);
            el.decorate();
            el.draw();
        });

    color_candidate
}
