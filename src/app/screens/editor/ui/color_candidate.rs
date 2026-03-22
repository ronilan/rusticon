use crate::{ui::reposition, State};
use little_tui::*;

static X: isize = 61;
static Y: isize = 11;

pub fn build() -> Element<State> {
    let color_candidate: Element<State> = Element::new();
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

            if let Some(bg) = color_source {
                el.background(Some(Color::Ansi(bg)));
            } else {
                el.background(None);
            }

            reposition(el, X, Y, state);
            el.decorate();
            el.draw();
        });

    color_candidate
}
