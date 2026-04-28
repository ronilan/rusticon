use crate::{core::model::State, ui::APP_HEIGHT};
use little_tui::*;
use little_tui_elements::Text;

pub fn build() -> Text<State> {
    let splash_footer: Text<State> = Text::default();
    splash_footer
        .y(APP_HEIGHT.saturating_sub(1) as isize)
        .text("Made with Rust")
        .faint(true)
        .bold(true);

    splash_footer
}
