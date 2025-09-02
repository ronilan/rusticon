use crate::tui_engine::*;
use crate::SplashState;

pub fn build<'a>() -> Element<'a, SplashState> {
    let mut splash_footer = Element::new(0, 0, Look::new());

    splash_footer.on_state = Some(Box::new(|el, _event| {
        let term_cols = columns();
        let term_rows = rows();

        let text = "Made with Rust";
        el.x.set((term_cols.saturating_sub(text.len() as u16)) / 2);
        el.y.set(term_rows.saturating_sub(1));
        el.look
            .update(terminal_style::format::bold(terminal_style::format::faint(
                Look::from(text),
            )));

        draw(el)
    }));

    splash_footer
}
