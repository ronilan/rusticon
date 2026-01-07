use crate::AppState;
use little_tui::*;

static X: i16 = 61;
static Y: i16 = 11;

pub fn build() -> Element<AppState> {
    let color_candidate: Element<AppState> = Element::new();
    color_candidate
        .x(X)
        .y(Y)
        .look(Look::from((15, 2, ' ')))
        .on_state(|el, state| {
            let color_source = if state.picker_mode {
                state.candidate
            } else {
                state.paintbrush
            };

            el.background(color_source);
            decorate(el);
            crate::ui::draw_relative(el, X, Y, state);
        });

    color_candidate
}
