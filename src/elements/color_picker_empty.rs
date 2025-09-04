use crate::elements::utils::*;
use crate::tui_engine::*;
use crate::AppState;

static X: u16 = 16;
static Y: u16 = 18;

pub fn build<'a>() -> Element<'a, AppState> {
    let mut color_picker_empty: Element<AppState> = Element::new(
        X,
        Y,
        Look::from(vec![
            vec![":".to_string(), ":".to_string()],
            vec![":".to_string(), ":".to_string()],
        ]),
    );

    color_picker_empty.on_move = Some(Box::new(|el, state, event| {
        if mouse_over(el, event) {
            state.candidate = None;
            state.picker_mode = true;
        }
    }));
    color_picker_empty.on_click = Some(Box::new(|el, state, event| {
        if mouse_over(el, event) {
            state.paintbrush = None;
            state.candidate = None;
            set_palette_in_state(state, state.candidate);
        }
    }));
    color_picker_empty.on_state = Some(Box::new(|el, state| {
        crate::elements::draw_relative(el, X, Y, state);
    }));

    color_picker_empty
}
