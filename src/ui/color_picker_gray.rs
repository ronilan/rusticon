use crate::ui::utils::*;
use crate::AppState;
use little_tui::*;

static X: i16 = 16;
static Y: i16 = 2;

pub fn build() -> Element<AppState> {
    let mut color_picker_gray: Element<AppState> = Element::new();
    color_picker_gray.x(X).y(Y).look(Look::from(
        (0..12)
            .map(|row| {
                (0..2)
                    .map(|col| {
                        let ansi_code: u8 = (row * 2 + col + 232).try_into().unwrap();
                        let decor = Decor::new(false, false, false, false, None, Some(ansi_code));
                        Block::new(' ', decor)
                    })
                    .collect::<Vec<Block>>()
            })
            .collect::<Vec<Vec<Block>>>(),
    ));

    color_picker_gray.listener.on_mouse = |el, state, event| {
        if event.mouse == Mouse::Move || event.mouse == Mouse::Click {
            let row = event.y.saturating_sub(el.visual.y.get()) as u8;
            let col = event.x.saturating_sub(el.visual.x.get()) as u8;

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
    };
    color_picker_gray.listener.on_state = |el, state| {
        crate::ui::draw_relative(el, X, Y, state);
    };

    color_picker_gray
}
