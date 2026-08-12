use incredible::*;
use incredible_elements::TextArea;

use crate::state::{State, validate};
use crate::ui::configure::shortcuts;

pub fn build_input_app_name() -> TextArea<State> {
    let mut input_app_name: TextArea<State> = TextArea::single();
    input_app_name
        .x(2)
        .y(1)
        .width(76)
        .label("App Name (Display Name)")
        .focused(true)
        .paste_enabled(true);

    shortcuts(&mut input_app_name);

    input_app_name
        .on_key(|el, state, _event| {
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
        })
        .on_mouse(|el, _state, event| {
            if event.mouse == Mouse::Click {
                el.focused(true);
            }
        })
        .on_window(|el, state, event| {
            if event.loop_count == 0 {
                el.text(&state.app_name);
            }
        })
        .on_change(|el, state, _event| {
            if el.status().focused.get() {
                state.focused_index = 0;
                let (ok, rule, inv) = validate(state);
                state.is_valid = ok;
                state.rule_text = rule.to_string();
                state.rule_invalid = inv;
            }
        });
    input_app_name
}
