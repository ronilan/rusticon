use crate::State;
use little_tui::*;
use little_tui_collection::Rectangle;

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
            let color_source = if state.picker_mode {
                state.candidate
            } else {
                state.paintbrush
            };

            el.background(color_source);
            el.draw();
        });

    color_candidate
}
