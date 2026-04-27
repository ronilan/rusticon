use crate::State;
use little_tui::*;
use little_tui_elements::Text;

static X: isize = 59;
static Y: isize = 19;

pub fn build() -> Text<State> {
    let label_end: Text<State> = Text::default();
    label_end.x(X).y(Y).text("End:");

    label_end
}
