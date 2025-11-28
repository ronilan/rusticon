use crate::AppState;
use little_tui::*;

static X: i16 = 67;
static Y: i16 = 2;

pub fn build() -> Element<AppState> {
    let mut button_8: Element<AppState> = Element::new();
    button_8
        .x(X)
        .y(Y)
        .look(terminal_style::format::underline(Look::from("8x8")));

    button_8.listener.on_mouse = |_el, state, event| {
        if event.mouse == Mouse::Click {
            state.size = 8;
            state.canvas8_data = vec![None; 64];

            // erase the 16x16 area
            // canvas 16 position
            static EX: i16 = 23;
            static EY: i16 = 3;
            let eraser: Element<AppState> = Element::new();
            eraser
                .x(EX)
                .y(EY)
                .look(Look::from(vec![vec![" ".to_string(); 32]; 16]));
            crate::ui::draw_relative(&eraser, EX, EY, state);
        }
    };
    button_8.listener.on_state = |el, state| {
        crate::ui::draw_relative(el, X, Y, state);
    };

    button_8
}
