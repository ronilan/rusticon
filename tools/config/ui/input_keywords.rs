use incredible::*;
use incredible_elements::TextArea;

use crate::state::{State, validate};

pub fn build_input_keywords(keywords: &str) -> TextArea<State> {
    let input_keywords: TextArea<State> = TextArea::input();
    input_keywords
        .x(2)
        .y(11)
        .width(76)
        .label("Keywords (comma,separated)")
        .text(keywords)
        .focused(false)
        .paste_enabled(true);
    input_keywords.on_key(|el, state, _event| {
        state.keywords = el.get_text();
        let (ok, rule, inv) = validate(state);
        state.is_valid = ok;
        state.rule_text = rule.to_string();
        state.rule_invalid = inv;
    });
    input_keywords.on_mouse(|_el, state, event| {
        if event.mouse == Mouse::Click {
            state.focused_index = 3;
            let (ok, rule, inv) = validate(state);
            state.is_valid = ok;
            state.rule_text = rule.to_string();
            state.rule_invalid = inv;
        }
    });
    input_keywords.on_state(|el, state| {
        el.focused(state.focused_index == 3);
    });
    input_keywords
}
