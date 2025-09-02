use crate::tui_engine::*;
use crate::elements::utils::*;
use crate::AppState;

const X: u16 = 1;
const Y: u16 = 2;

pub fn build<'a>() -> Element<'a, AppState> {
    let mut color_picker_16: Element<AppState> = Element::new(
        X,
        Y,
        Look::from(
            (0..16)
                .map(|row| {
                    (0..1)
                        .map(|_col| {
                            let ansi_code: u8 = terminal_style::color::rgb_to_ansi8(
                                terminal_style::color::ansi8_to_rgb(row),
                            )
                            .try_into()
                            .unwrap();
                            terminal_style::format::background(ansi_code, " ").unwrap()
                        })
                        .collect::<Vec<String>>()
                })
                .collect::<Vec<Vec<String>>>(),
        ),
    );

    color_picker_16.on_move = Some(Box::new(|el, state, event| {
        if mouse_over(el, event) {
            let row = event.y.unwrap().saturating_sub(el.y.get()) as u8;
            let ansi_code: u8 =
                terminal_style::color::rgb_to_ansi8(terminal_style::color::ansi8_to_rgb(row))
                    .try_into()
                    .unwrap();
            state.candidate = Some(ansi_code);
            state.picker_mode = true;
        }
    }));
    color_picker_16.on_click = Some(Box::new(|el, state, event| {
        if mouse_over(el, event) {
            let row = event.y.unwrap().saturating_sub(el.y.get()) as u8;
            let ansi_code: u8 =
                terminal_style::color::rgb_to_ansi8(terminal_style::color::ansi8_to_rgb(row))
                    .try_into()
                    .unwrap();
            state.paintbrush = Some(ansi_code);
            state.candidate = Some(ansi_code);
            set_palette_in_state(state, state.candidate);
        }
    }));
    color_picker_16.on_state = Some(Box::new(|el, state| {
        crate::elements::draw_relative(el, X, Y, state);
    }));

    color_picker_16
}
