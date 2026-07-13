use crate::core::model::State;
use incredible::*;
use incredible_elements::TextButton;

pub fn build() -> TextButton<State> {
    let button: TextButton<State> = TextButton::default();
    button
        .y(20)
        .pointer(Some(PointerShape::Pointer))
        .text("New Icon File")
        .underline(Some(UnderlineKind::Dotted));

    button.on_mouse(|_el, state, event| {
        if event.mouse == Mouse::Click {
            state.flow.launch_start_new = true;
            state.editor.file_handle = None;
            state.editor.file_path = "favicon.svg".to_string();
        }
    });

    button
}
