use incredible::*;
use incredible_elements::{FrameKind, ScrollArea};

use crate::state::State;

pub fn build_frame() -> ScrollArea<State> {
    let frame: ScrollArea<State> = ScrollArea::default();
    frame
        .width(80)
        .height(8)
        .kind(Some(FrameKind::Rounded))
        .label("Package")
        .next_combination(Some(KeyCombination::new(Key::Tab, &[])))
        .prev_combination(Some(KeyCombination::new(Key::BackTab, &[])))
        .cycle_next(true);

    frame
}
