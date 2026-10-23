use incredible_elements::{FrameKind, ScrollArea};

use crate::state::State;

pub fn build_frame(height: usize) -> ScrollArea<State> {
    let frame: ScrollArea<State> = ScrollArea::default();
    frame
        .width(38)
        .height(height)
        .kind(Some(FrameKind::Rounded))
        .focused_kind(Some(FrameKind::Rounded))
        .label("Run");

    frame
}
