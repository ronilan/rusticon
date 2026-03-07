use crate::State;
use little_tui::*;
use little_tui_collection::Text;

pub fn build() -> Text<State> {
    let hint: Text<State> = Text::default();
    hint.y(18).text("Drag-n-Drop File to Open").bold(true);

    hint
}
