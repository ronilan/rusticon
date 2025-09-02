use crate::tui_engine::*;
use crate::AppState;

pub fn build<'a>() -> Element<'a, AppState> {
    let mut title_bar: Element<AppState> = Element::new(0, 0, Look::new());

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
        draw(el);
    }));

    title_bar
}
