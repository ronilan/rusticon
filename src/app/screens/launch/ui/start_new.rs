use crate::State;
use little_tui::*;
use little_tui_collection::TextButton;

pub fn build() -> TextButton<State> {
    let button: TextButton<State> = TextButton::default();
    button.text("Start New Icon").y(20);

    button.on_mouse(|_el, state, event| {
        if event.mouse == Mouse::Click {
            state.flow.launch_start_new = true;
        }
    });

    button
}
