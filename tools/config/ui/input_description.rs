use incredible::*;
use incredible_elements::{TextArea, TextAreaOptions};

use crate::state::{State, validate};

pub fn build_input_description() -> TextArea<State> {
    let input_description: TextArea<State> = TextArea::new(TextAreaOptions {
        respond_to_enter: false,
        ..Default::default()
    });
    input_description
        .x(2)
        .y(13)
        .width(64)
        .height(6)
        .label("Description")
        .focused(false)
        .paste_enabled(true);

    input_description
        .on_key(|el, state, _event| {
            state.description = el.get_text();
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
                el.text(&state.description);
            }
        })
        .on_change(|el, state, _event| {
            if el.status().focused.get() {
                state.focused_index = 4;
                let (ok, rule, inv) = validate(state);
                state.is_valid = ok;
                state.rule_text = rule.to_string();
                state.rule_invalid = inv;
            }
        });
    input_description
}
