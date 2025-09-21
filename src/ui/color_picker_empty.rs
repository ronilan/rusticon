use crate::ui::utils::*;
use crate::AppState;
use little_tui::*;

static X: u16 = 16;
static Y: u16 = 18;

pub fn build<'a>() -> Element<'a, AppState> {
    let mut color_picker_empty: Element<AppState> = Element::new(
        Pos::new(X, Y),
        Look::from(vec![
            vec![":".to_string(), ":".to_string()],
            vec![":".to_string(), ":".to_string()],
        ]),
    );

    color_picker_empty.listener.on_move = Some(Box::new(|_el, state, _event| {
        state.candidate = None;
        state.picker_mode = true;
    }));
    color_picker_empty.listener.on_click = Some(Box::new(|_el, state, _event| {
        state.paintbrush = None;
        state.candidate = None;
        set_palette_in_state(state, state.candidate);
    }));
    color_picker_empty.listener.on_state = Some(Box::new(|el, state| {
        crate::ui::draw_relative(el, X, Y, state);
    }));

    color_picker_empty
}
