use crate::AppState;
use little_tui::engine::BaseElement;
use little_tui::*;

static X: u16 = 59;
static Y: u16 = 19;

pub fn build<'a>() -> BaseElement<'a, AppState> {
    let mut label_end: BaseElement<AppState> = BaseElement::new(Pos::new(X, Y), Look::from("End:"));

    label_end.on_state = Some(Box::new(|el, state| {
        crate::ui::draw_relative(el, X, Y, state);
    }));

    label_end
}
