use crate::tui_engine::*;
use crate::AppState;

static X: u16 = 59;
static Y: u16 = 19;

pub fn build<'a>() -> Element<'a, AppState> {
    let mut label_end: Element<AppState> = Element::new(X, Y, Look::from("End:"));

    label_end.on_state = Some(Box::new(|el, state| {
        crate::elements::draw_relative(el, X, Y, state);
    }));

    label_end
}
