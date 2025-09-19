use crate::AppState;
use little_tui::*;

static X: u16 = 67;
static Y: u16 = 2;

pub fn build<'a>() -> Element<'a, AppState> {
    let mut button_8: Element<AppState> = Element::new(
        Pos::new(X, Y),
        terminal_style::format::underline(Look::from("8x8")),
    );

    button_8.listener.on_click = Some(Box::new(|el, state, event| {
        if mouse_over(el, event) {
            state.size = 8;
            state.canvas8_data = vec![None; 64];

            // erase the 16x16 area
            // canvas 16 position
            static EX: u16 = 23;
            static EY: u16 = 3;
            let eraser: Element<AppState> = Element::new(
                Pos::new(EX, EY),
                Look::from(vec![vec![" ".to_string(); 32]; 16]),
            );
            crate::ui::draw_relative(&eraser, EX, EY, state);
        }
    }));
    button_8.listener.on_state = Some(Box::new(|el, state| {
        crate::ui::draw_relative(el, X, Y, state);
    }));

    button_8
}
