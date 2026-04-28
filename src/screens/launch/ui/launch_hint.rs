use crate::core::model::State;
use little_tui::*;
use little_tui_elements::Text;

pub fn build() -> Text<State> {
    let hint: Text<State> = Text::default();
    hint.y(2).text("Drag-n-Drop File to Open").bold(true);

    hint
}
