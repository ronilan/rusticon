use crate::State;
use little_tui::*;
use little_tui_collection::Rectangle;

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
            el.background(state.paintbrush);
            el.draw();
        });

    color_selected
}
