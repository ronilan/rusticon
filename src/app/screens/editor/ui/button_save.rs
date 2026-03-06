use crate::{
    core::model::{AppPhase, ExitFlow},
    State,
};
use little_tui::*;
use little_tui_collection::TextButton;

static X: isize = 75;
static Y: isize = 19;

pub fn build() -> TextButton<State> {
    let button_save: TextButton<State> = TextButton::default();
    button_save
        .x(X)
        .y(Y)
        .text("Save")
        .underline(true)
        .on_mouse(|_el, state, event| {
            if event.mouse == Mouse::Click {
                state.save_flag = true;
                state.phase = AppPhase::Message;
                state.message_text = Some("Saving...".to_string());
                state.message_color = 10;
                state.exit_flow = ExitFlow::SaveThenExit {
                    save_done: false,
                    started_ms: None,
                };
            }
        });

    button_save
}
