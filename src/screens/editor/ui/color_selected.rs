use crate::core::model::State;
use incredible::*;
use incredible_elements::Rectangle;
use incredible_helpers_styling::*;

static X: isize = 61;
static Y: isize = 9;

pub fn build() -> Rectangle<State> {
    let color_selected: Rectangle<State> = Rectangle::new();
    color_selected
        .x(X)
        .y(Y)
        .width(15)
        .height(2)
        .fill(Some(' '))
        .on_state(|el, state| {
            if let Some(bg) = state.editor.paintbrush {
                el.background(Some(Color::Ansi(bg)));
            } else {
                el.background(None);
            }
            el.draw();
        });

    color_selected
}
