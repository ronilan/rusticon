use crate::{
    core::model::AppPhase,
    ui::{APP_HEIGHT, APP_WIDTH},
    State,
};
use little_tui::*;
use little_tui_collection::Text;

pub fn build() -> Text<State> {
    let splash_footer: Text<State> = Text::default();
    let text = "Made with Rust".to_string();
    splash_footer
        .x((APP_WIDTH.saturating_sub(text.len())) as isize / 2)
        .y(APP_HEIGHT.saturating_sub(1) as isize)
        .text(&text)
        .faint(true)
        .bold(true);

    splash_footer.on_state(move |el, state| {
        if state.flow.phase != AppPhase::Splash && state.flow.phase != AppPhase::Launch {
            return;
        }
        if state.flow.viewport_too_small {
            return;
        }

        el.text(&text);
    });

    splash_footer
}
