use crate::{
    ui::{APP_HEIGHT, APP_WIDTH},
    State,
};
use little_tui::Mutators;
use little_tui_collection::Rectangle;

pub fn build() -> Rectangle<State> {
    let wrapper: Rectangle<State> = Rectangle::new();
    wrapper.width(APP_WIDTH).height(APP_HEIGHT).fill(Some(' '));

    wrapper.add(super::ui::splash_logo::build());
    wrapper.add(super::ui::splash_footer::build());

    wrapper
}
