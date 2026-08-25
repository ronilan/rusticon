use crate::{ui::reposition, State};
use incredible::*;

static X: isize = 61;
static Y: isize = 9;

pub fn build() -> Element<State> {
    let color_selected: Element<State> = Element::new();
    color_selected
        .x(X)
        .y(Y)
        .look(Look::from((15, 2, ' ')))
        .on_state(|el, state| {
            if let Some(bg) = state.paintbrush {
                el.decoration
                    .style
                    .base
                    .decor
                    .background
                    .set(Some(Color::Ansi(bg)));
            } else {
                el.decoration.style.base.decor.background.set(None);
            }

            reposition(el, X, Y, state);
            el.decorate();
            el.draw();
        });

    color_selected
}
