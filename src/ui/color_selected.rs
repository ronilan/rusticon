use crate::AppState;
use little_tui::engine::BaseElement;
use little_tui::*;

static X: u16 = 61;
static Y: u16 = 9;

pub fn build<'a>() -> BaseElement<'a, AppState> {
    let mut color_selected: BaseElement<AppState> = BaseElement::new(
        Pos::new(X, Y),
        Look::from(vec![vec![" ".to_string(); 15]; 2]),
    );

    color_selected.on_state = Some(Box::new(|el, state| {
        let look = Look::from(vec![vec![" ".to_string(); 15]; 2]);

        if let Some(pb) = state.paintbrush {
            let styled = terminal_style::format::background(pb, look).unwrap();
            el.look.update(styled);
        } else {
            el.look.update(look);
        }

        crate::ui::draw_relative(el, X, Y, state);
    }));

    color_selected
}
