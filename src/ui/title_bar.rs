use crate::ui::{APP_HEIGHT, APP_WIDTH};
use crate::AppState;
use little_tui::*;

pub fn build() -> Element<AppState> {
    let mut title_bar: Element<AppState> = Element::new();

    title_bar.listener.on_loop = |el, state, _event| {
        let x = (columns().saturating_sub(APP_WIDTH) / 2) as i16;
        let y = (rows().saturating_sub(APP_HEIGHT) / 2) as i16;

        if x != state.app_x || y != state.app_y {
            draw(el);
        }
    };

    title_bar.listener.on_state = |el, state| {
        let cols = columns() as usize;

        let mut line = " ".repeat(cols);
        let text = format!(
            "Rusticon: {} {}x{}",
            state.file_path, state.size, state.size
        );
        line.replace_range(0..text.len().min(cols), &text);

        el.look(Look::from(line)).inverse(true);
        decorate(el);
        draw(el);
    };

    title_bar
}
