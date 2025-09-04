use crate::tui_engine::*;
use crate::AppState;

static X: u16 = 75;
static Y: u16 = 19;

pub fn build<'a>() -> Element<'a, AppState> {
    let mut button_save: Element<AppState> =
        Element::new(X, Y, terminal_style::format::underline(Look::from("Save")));

    button_save.on_loop = Some(Box::new(|_el, state, _event| {
        if state.save_flag {
            // wait till next loop to exit
            state.exit_flag = true;
        }
    }));
    button_save.on_click = Some(Box::new(|el, state, event| {
        if mouse_over(el, event) {
            state.save_flag = true;
        }
    }));
    button_save.on_state = Some(Box::new(|el, state| {
        crate::elements::draw_relative(el, X, Y, state);
    }));

    button_save
}
