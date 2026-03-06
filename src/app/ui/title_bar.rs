use crate::ui::{APP_HEIGHT, APP_WIDTH};
use crate::State;
use little_tui::*;

pub fn build() -> Element<State> {
    let title_bar: Element<State> = Element::new();

    title_bar
        .on_loop(|el, state, _event| {
            let x = (Terminal::columns().saturating_sub(APP_WIDTH) / 2) as isize;
            let y = (Terminal::rows().saturating_sub(APP_HEIGHT) / 2) as isize;

            if x != state.app_x || y != state.app_y {
                el.draw();
            }
        })
        .on_state(|el, state| {
            let cols = Terminal::columns() as usize;

            let mut line = " ".repeat(cols);
            let text = format!(
                "Rusticon: {} {}x{}",
                state.file_path, state.size, state.size
            );
            line.replace_range(0..text.len().min(cols), &text);

            el.look(Look::from(line)).inverse(true);
            el.decorate();
            el.draw();
        });

    title_bar
}
