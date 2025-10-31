use crate::ui::utils::*;
use crate::AppState;
use little_tui::*;

static X: i16 = 1;
static Y: i16 = 2;

pub fn build() -> Element<AppState> {
    let mut color_picker_16: Element<AppState> = Element::new(
        Pos::new(X, Y),
        Look::from(
            (0..16)
                .map(|row| {
                    (0..1)
                        .map(|_col| {
                            let ansi_code: u8 = terminal_style::color::rgb_to_ansi8(
                                terminal_style::color::ansi8_to_rgb(row),
                            );
                            terminal_style::format::background(ansi_code, " ").unwrap()
                        })
                        .collect::<Vec<String>>()
                })
                .collect::<Vec<Vec<String>>>(),
        ),
    );

    color_picker_16.listener.on_mouse = Box::new(|el, state, event| {
        if event.mouse == Mouse::Move || event.mouse == Mouse::Click {
            let row = event.y.saturating_sub(el.pos.y.get()) as u8;
            let ansi_code: u8 =
                terminal_style::color::rgb_to_ansi8(terminal_style::color::ansi8_to_rgb(row));
            state.candidate = Some(ansi_code);

            if event.mouse == Mouse::Move {
                state.picker_mode = true;
            }
            if event.mouse == Mouse::Click {
                state.paintbrush = Some(ansi_code);
                set_palette_in_state(state, state.candidate);
            }
        }
    });
    color_picker_16.listener.on_state = Box::new(|el, state| {
        crate::ui::draw_relative(el, X, Y, state);
    });

    color_picker_16
}
