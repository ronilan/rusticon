use crate::AppState;
use little_tui::*;

static X: i16 = 67;
static Y: i16 = 19;

pub fn build() -> Element<AppState> {
    let mut button_exit: Element<AppState> = Element::new(
        Pos::new(X, Y),
        terminal_style::format::underline(Look::from("Exit")),
    );

    button_exit.listener.on_key = Some(Box::new(|_el, state, event| {
        if event.key == "c" && event.modifiers.contains(&"ctrl".to_string()) {
            state.exit_flag = true;
        }
    }));
    button_exit.listener.on_mouse = Some(Box::new(|_el, state, event| {
        if event.kind == "click" {
            state.exit_flag = true;
        }
    }));
    button_exit.listener.on_state = Some(Box::new(|el, state| {
        crate::ui::draw_relative(el, X, Y, state);
    }));

    button_exit
}
