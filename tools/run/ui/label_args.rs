use incredible::*;
use incredible_elements::FramedText;

use crate::state::{State, TargetPlatform};

pub fn build_label_args(y: usize) -> FramedText<State> {
    let label_args: FramedText<State> = FramedText::default();
    label_args.conf_unframed();
    label_args
        .x(1)
        .y(y as isize)
        .width(34)
        .height(1)
        .ellipsis(true)
        .wrap(false)
        .focused(false)
        .attention_keys(AttentionLevel::Never);

    label_args
        .on_window(|el, state, event| {
            // Text is fixed at init — paint it on the first frame.
            if event.loop_count == 0 {
                if state.args_text.is_empty() {
                    el.text("Args: (none)");
                } else {
                    el.text(&format!("Args: {}", state.args_text));
                }
            }
        })
        .on_state(|el, state, _event| {
            // The wasm target cannot take args — dim the text while selected.
            // Guarded: unguarded draw() here re-enters on_state and never settles.
            let dimmed = matches!(state.selected_target, Some(TargetPlatform::Wasm));
            if dimmed != el.status().disabled.get() {
                el.disabled(dimmed);
                el.draw();
            }
        });
    label_args
}
