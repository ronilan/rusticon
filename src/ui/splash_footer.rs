use crate::SplashState;
use little_tui::engine::{draw_base, BaseElement};
use little_tui::*;

pub fn build<'a>() -> BaseElement<'a, SplashState> {
    let mut splash_footer = BaseElement::new(Pos::new(0, 0), Look::new());

    splash_footer.on_state = Some(Box::new(|el, _event| {
        let term_cols = columns();
        let term_rows = rows();

        let text = "Made with Rust";
        el.pos
            .x
            .set((term_cols.saturating_sub(text.len() as u16)) / 2);
        el.pos.y.set(term_rows.saturating_sub(1));
        el.look
            .update(terminal_style::format::bold(terminal_style::format::faint(
                Look::from(text),
            )));

        draw_base(el)
    }));

    splash_footer
}
