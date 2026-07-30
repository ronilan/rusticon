use incredible::*;
use incredible_elements::TextArea;

use crate::state::{State, validate};

pub fn build_input_app_name(app_name: &str) -> TextArea<State> {
    let input_app_name: TextArea<State> = TextArea::input();
    input_app_name
        .x(2)
        .y(2)
        .width(76)
        .label("App Name (Display Name)")
        .text(app_name)
        .focused(true)
        .paste_enabled(true);
    input_app_name.on_key(|el, state, _event| {
        state.app_name = el.get_text();
        // One-way sync: App Name → Name (snake_case)
        state.name = state
            .app_name
            .chars()
            .map(|c| {
                if c.is_alphanumeric() {
                    c.to_ascii_lowercase()
                } else if c.is_whitespace() {
                    '_'
                } else {
                    '\0'
                }
            })
            .filter(|c| *c != '\0')
            .collect();
        let (ok, rule, inv) = validate(state);
        state.is_valid = ok;
        state.rule_text = rule.to_string();
        state.rule_invalid = inv;
    });
    input_app_name.on_mouse(|_el, state, event| {
        if event.mouse == Mouse::Click {
            state.focused_index = 0;
            let (ok, rule, inv) = validate(state);
            state.is_valid = ok;
            state.rule_text = rule.to_string();
            state.rule_invalid = inv;
        }
    });
    input_app_name.on_state(|el, state| {
        el.focused(state.focused_index == 0);
    });
    input_app_name
}
