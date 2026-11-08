use crate::core::model::State;
use incredible::*;
use incredible_elements::Button;
use incredible_helpers_styling::*;

pub fn build() -> Button<State> {
    let button: Button<State> = Button::default();
    button
        .y(20)
        .focused(false)
        .pointer(Some(PointerShape::Pointer))
        .text("New Icon File")
        .underline(Some(UnderlineKind::Dotted))
        .on_mouse(|_el, state, event| {
            if event.mouse == Mouse::Click {
                state.flow.launch_start_new = true;
                state.editor.file_handle = None;
                state.editor.file_path = "favicon.svg".to_string();
            }
        });

    button
}
