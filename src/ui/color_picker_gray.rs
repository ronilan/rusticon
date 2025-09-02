use crate::tui_engine::*;
use crate::ui::utils::*;
use crate::AppState;

const X: u16 = 16;
const Y: u16 = 2;

pub fn build<'a>() -> Element<'a, AppState> {
    let mut color_picker_gray: Element<AppState> = Element::new(
        X,
        Y,
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
    color_picker_gray.on_move = Some(Box::new(|el, state, event| {
        if mouse_over(el, event) {
            let row = event.y.unwrap().saturating_sub(el.y.get()) as u8;
            let col = event.x.unwrap().saturating_sub(el.x.get()) as u8;
            let code = (row * 2 + col + 232) as u8;
            state.candidate = Some(code);
            state.picker_mode = true;
        }
    }));
    color_picker_gray.on_click = Some(Box::new(|el, state, event| {
        if mouse_over(el, event) {
            let row = event.y.unwrap().saturating_sub(el.y.get()) as u8;
            let col = event.x.unwrap().saturating_sub(el.x.get()) as u8;
            let code = (row * 2 + col + 232) as u8;
            state.paintbrush = Some(code);
            state.candidate = Some(code);
            set_palette_in_state(state, state.candidate);
        }
    }));
    color_picker_gray.on_state = Some(Box::new(|el, state| {
        crate::ui::draw_relative(el, X, Y, state);
    }));

    color_picker_gray
}
