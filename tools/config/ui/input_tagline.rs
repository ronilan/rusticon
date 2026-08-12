use incredible::*;
use incredible_elements::TextArea;

use crate::state::{State, validate};
use crate::ui::configure::shortcuts;

pub fn build_input_tagline() -> TextArea<State> {
    let mut input_tagline: TextArea<State> = TextArea::single();
    input_tagline
        .x(2)
        .y(7)
        .width(76)
        .label("Tagline (HTML title suffix)")
        .focused(false)
        .paste_enabled(true);

    shortcuts(&mut input_tagline);

    input_tagline
        .on_key(|el, state, _event| {
            state.tagline = el.get_text();
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
                el.text(&state.tagline);
            }
        })
        .on_change(|el, state, _event| {
            if el.status().focused.get() {
                state.focused_index = 2;
                let (ok, rule, inv) = validate(state);
                state.is_valid = ok;
                state.rule_text = rule.to_string();
                state.rule_invalid = inv;
            }
        });
    input_tagline
}
