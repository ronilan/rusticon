use crate::{
    ui::{APP_HEIGHT, APP_WIDTH},
    State,
};
use little_tui::{Mutators, Setters};
use little_tui_collection::Rectangle;

pub fn build() -> Rectangle<State> {
    let wrapper: Rectangle<State> = Rectangle::new();
    wrapper
        .x(0)
        .y(1)
        .width(APP_WIDTH)
        .height(APP_HEIGHT.saturating_sub(1))
        .fill(Some(' '));

    wrapper.add(super::ui::message::build());

    wrapper
}
