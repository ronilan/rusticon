use incredible::*;
use incredible_elements::{Button, Frame, FrameKind, Select};

use crate::state::State;

pub fn build_frame() -> Frame<State> {
    let frame: Frame<State> = Frame::new();
    frame.width(80).height(8).kind(Some(FrameKind::Rounded));

    frame.on_key(|el, state, event| {
        if event.key == Key::Tab {
            let selects = el.elements.cot::<Select<State>>();
            let buttons = el.elements.cot::<Button<State>>();
            let total = selects.len() + buttons.len();
            if total == 0 {
                return;
            }

            for _ in 0..total {
                state.focused_index = (state.focused_index + 1) % total;
                let disabled = if state.focused_index < selects.len() {
                    selects[state.focused_index].get_disabled()
                } else {
                    buttons[state.focused_index - selects.len()]
                        .status()
                        .disabled
                        .get()
                };
                if !disabled {
                    break;
                }
            }

            for (i, sel) in selects.iter().enumerate() {
                sel.focused(i == state.focused_index);
            }
            for (i, btn) in buttons.iter().enumerate() {
                btn.focused(i + selects.len() == state.focused_index);
            }
        }
    });

    frame
}
