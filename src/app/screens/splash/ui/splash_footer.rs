use crate::{
    core::model::AppPhase,
    ui::{APP_HEIGHT, APP_WIDTH},
    State,
};
use little_tui::*;
use little_tui_collection::Text;

pub fn build() -> Text<State> {
    let splash_footer: Text<State> = Text::default();
    splash_footer
        .y(APP_HEIGHT.saturating_sub(1) as isize)
        .text("Made with Rust")
        .faint(true)
        .bold(true);

    splash_footer
}
