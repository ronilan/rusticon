use incredible::*;
use incredible_elements::{TextArea, TextAreaOptions};

use crate::state::{State, validate};
use crate::ui::configure::shortcuts;

pub fn build_input_name() -> TextArea<State> {
    let mut input_name: TextArea<State> = TextArea::new(TextAreaOptions {
        max_length: 64,
        ..TextAreaOptions::single()
    });
    input_name
        .x(2)
        .y(4)
        .width(76)
        .height(3)
        .wrap(false)
        .label("Name (snake_case)")
        .focused(false)
        .paste_enabled(true);

    shortcuts(&mut input_name);

    input_name
        .on_key(|el, state, _event| {
            state.name = el.get_text();
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
                el.text(&state.name);
            }
        })
        .on_change(|el, state, _event| {
            if el.status().focused.get() {
                state.focused_index = 1;
                let (ok, rule, inv) = validate(state);
                state.is_valid = ok;
                state.rule_text = rule.to_string();
                state.rule_invalid = inv;
            }
        });
    input_name.on_state(|el, state| {
        if !el.status().focused.get() {
            el.text(&state.name);
        }
    });
    input_name
}
