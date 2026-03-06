use crate::{core::shared::RESULT_HOLDER, SplashState, MIN_SPLASH_LOOPS};
use little_tui::*;

pub fn build() -> Element<SplashState> {
    let splash_footer = Element::new();

    splash_footer.on_state(|el: &Element<SplashState>, state| {
        let term_cols = Terminal::columns() as isize;
        let term_rows = Terminal::rows() as isize;

        let text = "Made with Rust";
        el.x((term_cols.saturating_sub(text.len() as isize)) / 2)
            .y(term_rows.saturating_sub(1))
            .look(Look::from(text))
            .faint(true)
            .bold(true);

        el.draw();

        // --- check background thread completion ---
        if let Ok(guard) = RESULT_HOLDER.try_lock() {
            let done = guard.is_some();
            if done && state.loop_count >= MIN_SPLASH_LOOPS {
                exit();
            }
        }
    });

    splash_footer
}
