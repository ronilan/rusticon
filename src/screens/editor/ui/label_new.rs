use crate::core::model::State;
use little_tui::*;
use little_tui_elements::Text;

static X: isize = 59;
static Y: isize = 2;

pub fn build() -> Text<State> {
    let label_new: Text<State> = Text::default();
    label_new.x(X).y(Y).text("New:");

    label_new
}
