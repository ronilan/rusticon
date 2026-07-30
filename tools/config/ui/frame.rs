use incredible::*;
use incredible_elements::{Button, Frame, FrameKind, TextArea};

use crate::state::{State, validate};

pub fn build_frame() -> Frame<State> {
    let frame: Frame<State> = Frame::new();
    frame.width(80).height(24).kind(Some(FrameKind::Rounded));

    frame.on_key(|el, state, event| {
        if event.key == Key::Tab {
            let inputs = el.elements.cot::<TextArea<State>>();
            let buttons = el.elements.cot::<Button<State>>();
            let total = inputs.len() + buttons.len();
            if total == 0 {
                return;
            }

            for _ in 0..total {
                state.focused_index = (state.focused_index + 1) % total;
                let disabled = if state.focused_index < inputs.len() {
                    inputs[state.focused_index].status().disabled.get()
                } else {
                    buttons[state.focused_index - inputs.len()]
                        .status()
                        .disabled
                        .get()
                };
                if !disabled {
                    break;
                }
            }

            for (i, inp) in inputs.iter().enumerate() {
                inp.focused(i == state.focused_index);
            }
            for (i, btn) in buttons.iter().enumerate() {
                btn.focused(i + inputs.len() == state.focused_index);
            }

            let (ok, rule, inv) = validate(state);
            state.is_valid = ok;
            state.rule_text = rule.to_string();
            state.rule_invalid = inv;
        }
    });

    frame
}
