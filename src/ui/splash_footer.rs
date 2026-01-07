use crate::{shared::RESULT_HOLDER, SplashState, MIN_SPLASH_LOOPS};
use little_tui::*;

pub fn build() -> Element<SplashState> {
    let splash_footer = Element::new();

    splash_footer
        .listener
        .on_state(|el: &Element<SplashState>, state| {
            let term_cols = Terminal::columns() as i16;
            let term_rows = Terminal::rows() as i16;

            let text = "Made with Rust";
            el.x((term_cols.saturating_sub(text.len() as i16)) / 2)
                .y(term_rows.saturating_sub(1))
                .look(Look::from(text))
                .faint(true)
                .bold(true);

            draw(el);

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
