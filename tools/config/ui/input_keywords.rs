use incredible::*;
use incredible_elements::TextArea;

use crate::state::{State, validate};
use crate::ui::configure::shortcuts;

pub fn build_input_keywords() -> TextArea<State> {
    let mut input_keywords: TextArea<State> = TextArea::single();
    input_keywords
        .x(2)
        .y(10)
        .width(76)
        .label("Keywords (comma,separated)")
        .focused(false)
        .paste_enabled(true);

    shortcuts(&mut input_keywords);

    input_keywords
        .on_key(|el, state, _event| {
            state.keywords = el.get_text();
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
                el.text(&state.keywords);
            }
        })
        .on_change(|el, state, _event| {
            if el.status().focused.get() {
                state.focused_index = 3;
                let (ok, rule, inv) = validate(state);
                state.is_valid = ok;
                state.rule_text = rule.to_string();
                state.rule_invalid = inv;
            }
        });
    input_keywords
}
