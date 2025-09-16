use crate::AppState;
use little_tui::engine::{draw_base, BaseElement};
use little_tui::*;

pub fn build<'a>() -> BaseElement<'a, AppState> {
    let mut title_bar: BaseElement<AppState> = BaseElement::new(Pos::new(0, 0), Look::new());

    title_bar.on_state = Some(Box::new(move |el, state| {
        let cols = crossterm::terminal::size().unwrap().0 as usize;

        let mut line = " ".repeat(cols);
        let text = format!(
            "Rusticon: {} {}x{}",
            state.file_path, state.size, state.size
        );
        line.replace_range(0..text.len().min(cols), &text);

        el.look
            .update(vec![vec![terminal_style::format::inverse(&line)]]);
        draw_base(el);
    }));

    title_bar
}
