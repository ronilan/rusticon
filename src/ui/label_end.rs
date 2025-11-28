use crate::AppState;
use little_tui::*;

static X: i16 = 59;
static Y: i16 = 19;

pub fn build() -> Element<AppState> {
    let mut label_end: Element<AppState> = Element::new();
    label_end.x(X).y(Y).look(Look::from("End:"));

    label_end.listener.on_state = |el, state| {
        crate::ui::draw_relative(el, X, Y, state);
    };

    label_end
}
