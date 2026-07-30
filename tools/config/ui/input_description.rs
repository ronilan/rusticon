use incredible::*;
use incredible_elements::{TextArea, TextAreaOptions};

use crate::state::{State, validate};

pub fn build_input_description(description: &str) -> TextArea<State> {
    let input_description: TextArea<State> = TextArea::new(TextAreaOptions {
        respond_to_enter: false,
        ..Default::default()
    });
    input_description
        .x(2)
        .y(14)
        .width(64)
        .height(6)
        .label("Description")
        .text(description)
        .focused(false)
        .paste_enabled(true);
    input_description.on_key(|el, state, _event| {
        state.description = el.get_text();
        let (ok, rule, inv) = validate(state);
        state.is_valid = ok;
        state.rule_text = rule.to_string();
        state.rule_invalid = inv;
    });
    input_description.on_mouse(|_el, state, event| {
        if event.mouse == Mouse::Click {
            state.focused_index = 4;
            let (ok, rule, inv) = validate(state);
            state.is_valid = ok;
            state.rule_text = rule.to_string();
            state.rule_invalid = inv;
        }
    });
    input_description.on_state(|el, state| {
        el.focused(state.focused_index == 4);
    });
    input_description
}
