use incredible::*;
use incredible_elements::Label;
use incredible_helpers_styling::StyleSetters;

use crate::state::State;

pub fn build_label_rule() -> Label<State> {
    let label_rule: Label<State> = Label::default();
    label_rule
        .x(2)
        .y(20)
        .wrap_at(76)
        .focused(false)
        .interactive(false);

    label_rule.on_state(|el, state| {
        el.text(&format!(" {}", state.rule_text));
        if state.rule_invalid {
            el.color(Some(Color::from(1)));
        } else {
            el.color(None);
        }
    });
    label_rule
}
