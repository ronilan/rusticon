use crate::{ui::reposition, AppState};
use little_tui::*;

static X: isize = 59;
static Y: isize = 2;

pub fn build() -> Element<AppState> {
    let label_new: Element<AppState> = Element::new();
    label_new
        .x(X)
        .y(Y)
        .look(Look::from("New:"))
        .on_state(|el, state| {
            reposition(el, X, Y, state);
            el.draw();
        });

    label_new
}
