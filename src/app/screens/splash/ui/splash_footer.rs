use crate::{core::shared::RESULT_HOLDER, SplashState, MIN_SPLASH_MS};
use little_tui::*;

pub fn build() -> Element<SplashState> {
    let splash_footer = Element::new();

    splash_footer.on_loop(|el: &Element<SplashState>, state, _event| {
        if state.started_ms.is_none() {
            state.started_ms = Some(Globals::now());
        }

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
            let elapsed = state
                .started_ms
                .map(|start| Globals::now() - start)
                .unwrap_or(0.0);
            if done && elapsed >= MIN_SPLASH_MS {
                exit();
            }
        }
    });

    splash_footer
}
