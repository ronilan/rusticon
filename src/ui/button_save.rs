use crate::AppState;
use little_tui::*;

static X: i16 = 75;
static Y: i16 = 19;

pub fn build() -> Element<AppState> {
    let mut button_save: Element<AppState> = Element::new();
    button_save
        .x(X)
        .y(Y)
        .look(Look::from("Save"))
        .underline(true);

    button_save.listener.on_loop = |_el, state, _event| {
        if state.save_flag {
            // wait till next loop to exit
            exit();
        }
    };
    button_save.listener.on_mouse = |_el, state, event| {
        if event.mouse == Mouse::Click {
            state.save_flag = true;
        }
    };
    button_save.listener.on_state = |el, state| {
        decorate(el);
        crate::ui::draw_relative(el, X, Y, state);
    };

    button_save
}
