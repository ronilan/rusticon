use crate::{shared::RESULT_HOLDER, SplashState, MIN_SPLASH_LOOPS};
use little_tui::*;

pub fn build() -> Element<SplashState> {
    let mut splash_footer = Element::new(Pos::new(0, 0), Look::new());

    splash_footer.listener.on_state = Some(Box::new(|el: &Element<SplashState>, state| {
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

        draw(el);

        // --- check background thread completion ---
        if let Ok(guard) = RESULT_HOLDER.try_lock() {
            let done = guard.is_some();
            if done && state.loop_count >= MIN_SPLASH_LOOPS {
                exit();
            }
        }
    }));

    splash_footer
}
