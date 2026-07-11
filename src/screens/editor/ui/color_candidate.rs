use crate::core::model::State;
use incredible::*;
use incredible_elements::Rectangle;

static X: isize = 61;
static Y: isize = 11;

pub fn build() -> Rectangle<State> {
    let color_candidate: Rectangle<State> = Rectangle::new();
    color_candidate
        .x(X)
        .y(Y)
        .width(15)
        .height(2)
        .fill(Some(' '))
        .on_state(|el, state| {
            let color_source = if state.editor.picker_mode {
                state.editor.candidate
            } else {
                state.editor.paintbrush
            };

            if let Some(bg) = color_source {
                el.background(Some(Color::Ansi(bg)));
            } else {
                el.background(None);
            }
            el.draw();
        });

    color_candidate
}
