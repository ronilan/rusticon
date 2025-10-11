use crate::SplashState;
use little_tui::*;

pub fn build() -> Element<SplashState> {
    let mut splash_footer = Element::new(Pos::new(0, 0), Look::new());

    splash_footer.listener.on_state = Some(Box::new(|el, _event| {
        let term_cols = columns() as i16;
        let term_rows = rows() as i16;

        let text = "Made with Rust";
        el.pos
            .x
            .set((term_cols.saturating_sub(text.len() as i16)) / 2);
        el.pos.y.set(term_rows.saturating_sub(1));
        el.look
            .update(terminal_style::format::bold(terminal_style::format::faint(
                Look::from(text),
            )));

        draw(el)
    }));

    splash_footer
}
