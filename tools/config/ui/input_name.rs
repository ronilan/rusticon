use incredible::*;
use incredible_elements::{TextArea, TextAreaOptions};

use crate::state::{State, validate};

pub fn build_input_name(name: &str) -> TextArea<State> {
    let input_name: TextArea<State> = TextArea::new(TextAreaOptions {
        max_length: 64,
        ..TextAreaOptions::input()
    });
    input_name
        .x(2)
        .y(5)
        .width(76)
        .height(3)
        .wrap(false)
        .label("Name (snake_case)")
        .text(name)
        .focused(false)
        .paste_enabled(true);
    input_name.on_key(|el, state, _event| {
        state.name = el.get_text();
        let (ok, rule, inv) = validate(state);
        state.is_valid = ok;
        state.rule_text = rule.to_string();
        state.rule_invalid = inv;
    });
    input_name.on_mouse(|_el, state, event| {
        if event.mouse == Mouse::Click {
            state.focused_index = 1;
            let (ok, rule, inv) = validate(state);
            state.is_valid = ok;
            state.rule_text = rule.to_string();
            state.rule_invalid = inv;
        }
    });
    input_name.on_state(|el, state| {
        el.focused(state.focused_index == 1);
        if state.focused_index != 1 {
            el.text(&state.name);
        }
    });
    input_name
}
