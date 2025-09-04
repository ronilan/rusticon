use crate::tui_engine::*;
use crate::AppState;

static X: u16 = 67;
static Y: u16 = 19;

pub fn build<'a>() -> Element<'a, AppState> {
    let mut button_exit: Element<AppState> =
        Element::new(X, Y, terminal_style::format::underline(Look::from("Exit")));

    button_exit.on_keypress = Some(Box::new(|_el, state, event| {
        if event.key == Some("c".to_string()) && event.modifiers.contains(&"ctrl".to_string()) {
            state.exit_flag = true;
        }
    }));
    button_exit.on_click = Some(Box::new(|el, state, event| {
        if mouse_over(el, event) {
            state.exit_flag = true;
        }
    }));
    button_exit.on_state = Some(Box::new(|el, state| {
        crate::elements::draw_relative(el, X, Y, state);
    }));

    button_exit
}
