use crate::AppState;
use little_tui::*;

static X: i16 = 75;
static Y: i16 = 19;

pub fn build() -> Element<AppState> {
    let mut button_save: Element<AppState> = Element::new(
        Pos::new(X, Y),
        terminal_style::format::underline(Look::from("Save")),
    );

    button_save.listener.on_loop = Some(Box::new(|_el, state, _event| {
        if state.save_flag {
            // wait till next loop to exit
            exit();
        }
    }));
    button_save.listener.on_mouse = Some(Box::new(|_el, state, event| {
        if event.mouse == Mouse::Click {
            state.save_flag = true;
        }
    }));
    button_save.listener.on_state = Some(Box::new(|el, state| {
        crate::ui::draw_relative(el, X, Y, state);
    }));

    button_save
}
