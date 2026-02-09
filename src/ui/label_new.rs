use crate::AppState;
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
            crate::ui::draw_relative(el, X, Y, state);
        });

    label_new
}
