use incredible::*;
use incredible_elements::TextArea;

use crate::state::{State, validate};

pub fn build_input_tagline(tagline: &str) -> TextArea<State> {
    let input_tagline: TextArea<State> = TextArea::input();
    input_tagline
        .x(2)
        .y(8)
        .width(76)
        .label("Tagline (HTML title suffix)")
        .text(tagline)
        .focused(false)
        .paste_enabled(true);
    input_tagline.on_key(|el, state, _event| {
        state.tagline = el.get_text();
        let (ok, rule, inv) = validate(state);
        state.is_valid = ok;
        state.rule_text = rule.to_string();
        state.rule_invalid = inv;
    });
    input_tagline.on_mouse(|_el, state, event| {
        if event.mouse == Mouse::Click {
            state.focused_index = 2;
            let (ok, rule, inv) = validate(state);
            state.is_valid = ok;
            state.rule_text = rule.to_string();
            state.rule_invalid = inv;
        }
    });
    input_tagline.on_state(|el, state| {
        el.focused(state.focused_index == 2);
    });
    input_tagline
}
