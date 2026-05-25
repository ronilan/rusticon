use crate::{
    core::model::{AppPhase, State},
    ui::{APP_HEIGHT, APP_WIDTH},
};
use incredible::*;
use incredible_elements::{Rectangle, Text};
use incredible_helpers_layout::arrangers::Arrangers;

pub fn build() -> Rectangle<State> {
    let message: Rectangle<State> = Rectangle::new();
    message
        .width(APP_WIDTH)
        .height(APP_HEIGHT.saturating_sub(1))
        .fill(Some(' '));

    let text: Text<State> = Text::default();
    text.text("App Terminated");
    message.add(text);
    message.elements_to_center();

    message.on_state(|el, state| {
        if state.flow.phase != AppPhase::Message {
            return;
        }

        let text_value = state
            .flow
            .message_text
            .clone()
            .unwrap_or_else(|| "App Terminated".to_string());

        if let Some(text_el) = el.elements().collect_of_type::<Text<State>>().first() {
            text_el
                .text(&text_value)
                .bold(true)
                .color(Some(Color::Ansi(state.flow.message_color)));
        }

        el.elements_to_center();
        el.draw();
    });

    message
}
