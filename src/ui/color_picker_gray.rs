use crate::ui::utils::*;
use crate::AppState;
use little_tui::*;

static X: i16 = 16;
static Y: i16 = 2;

pub fn build() -> Element<AppState> {
    let mut color_picker_gray: Element<AppState> = Element::new(
        Pos::new(X, Y),
        Look::from(
            (0..12)
                .map(|row| {
                    (0..2)
                        .map(|col| {
                            let code: u8 = (row * 2 + col + 232).try_into().unwrap();
                            terminal_style::format::background(code, " ").unwrap()
                        })
                        .collect::<Vec<String>>() // a single row
                })
                .collect::<Vec<Vec<String>>>(), // all rows
        ),
    );

    color_picker_gray.listener.on_mouse = Some(Box::new(|el, state, event| {
        if event.mouse == Mouse::Move || event.mouse == Mouse::Click {
            let row = event.coords.y.get().saturating_sub(el.pos.y.get()) as u8;
            let col = event.coords.x.get().saturating_sub(el.pos.x.get()) as u8;

            let ansi_code = row * 2 + col + 232;
            state.candidate = Some(ansi_code);
            state.picker_mode = true;

            if event.mouse == Mouse::Move {
                state.picker_mode = true;
            }
            if event.mouse == Mouse::Click {
                state.paintbrush = Some(ansi_code);
                set_palette_in_state(state, state.candidate);
            }
        }
    }));
    color_picker_gray.listener.on_state = Some(Box::new(|el, state| {
        crate::ui::draw_relative(el, X, Y, state);
    }));

    color_picker_gray
}
