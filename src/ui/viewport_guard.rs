use crate::State;
use little_tui::*;
use little_tui_collection::{Rectangle, Text};

pub fn build() -> Rectangle<State> {
    let guard: Rectangle<State> = Rectangle::new();
    guard.x(0).y(0).fused(true).showed(false).fill(Some(' '));
    guard.width(Terminal::columns()).height(Terminal::rows());

    let text: Text<State> = Text::default();
    text.text("Enlarge Terminal Window");
    guard.add(text);
    guard.elements_to_center();

    guard.on_window(|el: &Rectangle<State>, _state, event| {
        if event.window == Window::Resize {
            el.width(event.columns).height(event.rows);
            el.elements_to_center();
            if el.get_showed() {
                el.draw();
            }
        }
    });

    guard
}
