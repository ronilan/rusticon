use crate::AppState;
use little_tui::*;

static X: i16 = 59;
static Y: i16 = 2;

pub fn build() -> Element<AppState> {
    let mut label_new: Element<AppState> = Element::new(Pos::new(X, Y), Look::from("New:"));

    label_new.listener.on_state = |el, state| {
        crate::ui::draw_relative(el, X, Y, state);
    };

    label_new
}
