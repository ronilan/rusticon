use crate::elements::utils::*;
use crate::tui_engine::*;
use crate::AppState;

static X: u16 = 3;
static Y: u16 = 2;

pub fn build<'a>() -> Element<'a, AppState> {
    let mut color_picker_216: Element<AppState> = Element::new(
        X,
        Y,
        Look::from(
            (0..18)
                .map(|row| {
                    (0..12)
                        .map(|col| {
                            // original crumb formula: (row * 12) + (col * 16)
                            let code = (row * 12 + col + 16).min(231) as u8;
                            terminal_style::format::background(code, " ").unwrap()
                        })
                        .collect::<Vec<String>>()
                })
                .collect::<Vec<Vec<String>>>(),
        ),
    );
    color_picker_216.on_move = Some(Box::new(|el, state, event| {
        if mouse_over(el, event) {
            let row = event.y.unwrap().saturating_sub(el.y.get()) as u8;
            let col = event.x.unwrap().saturating_sub(el.x.get()) as u8;
            let code = row * 12 + col + 16;
            state.candidate = Some(code);
            state.picker_mode = true;
        }
    }));
    color_picker_216.on_click = Some(Box::new(|el, state, event| {
        if mouse_over(el, event) {
            let row = event.y.unwrap().saturating_sub(el.y.get()) as u8;
            let col = event.x.unwrap().saturating_sub(el.x.get()) as u8;
            let code = row * 12 + col + 16;
            state.paintbrush = Some(code);
            state.candidate = Some(code);
            set_palette_in_state(state, state.candidate);
        }
    }));
    color_picker_216.on_state = Some(Box::new(|el, state| {
        crate::elements::draw_relative(el, X, Y, state);
    }));

    color_picker_216
}
