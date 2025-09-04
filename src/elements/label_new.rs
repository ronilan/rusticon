use crate::tui_engine::*;
use crate::AppState;

static X: u16 = 59;
static Y: u16 = 2;

pub fn build<'a>() -> Element<'a, AppState> {
    let mut label_new: Element<AppState> = Element::new(X, Y, Look::from("New:"));

    label_new.on_state = Some(Box::new(|el, state| {
        crate::elements::draw_relative(el, X, Y, state);
    }));

    label_new
}
