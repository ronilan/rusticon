use crate::AppState;
use little_tui::engine::BaseElement;
use little_tui::*;

static X: u16 = 59;
static Y: u16 = 2;

pub fn build<'a>() -> BaseElement<'a, AppState> {
    let mut label_new: BaseElement<AppState> = BaseElement::new(Pos::new(X, Y), Look::from("New:"));

    label_new.on_state = Some(Box::new(|el, state| {
        crate::ui::draw_relative(el, X, Y, state);
    }));

    label_new
}
