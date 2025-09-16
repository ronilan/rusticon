use crate::AppState;
use little_tui::engine::{mouse_over_base, BaseElement};
use little_tui::*;

static X: u16 = 75;
static Y: u16 = 19;

pub fn build<'a>() -> BaseElement<'a, AppState> {
    let mut button_save: BaseElement<AppState> = BaseElement::new(
        Pos::new(X, Y),
        terminal_style::format::underline(Look::from("Save")),
    );

    button_save.on_loop = Some(Box::new(|_el, state, _event| {
        if state.save_flag {
            // wait till next loop to exit
            state.exit_flag = true;
        }
    }));
    button_save.on_click = Some(Box::new(|el, state, event| {
        if mouse_over_base(el, event) {
            state.save_flag = true;
        }
    }));
    button_save.on_state = Some(Box::new(|el, state| {
        crate::ui::draw_relative(el, X, Y, state);
    }));

    button_save
}
