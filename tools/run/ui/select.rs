use incredible::*;
use incredible_elements::{FrameKind, Select};

use crate::{
    platform::{is_macos, is_windows},
    state::{State, TargetPlatform},
};

pub fn build_select(select_height: usize) -> Select<State> {
    let select = Select::<State>::default();
    select
        .x(1)
        .y(1)
        .width(36)
        .height(select_height)
        .frame_kind(Some(FrameKind::Rounded))
        .focused_frame_kind(Some(FrameKind::Rounded))
        .label("Platform")
        .add_item("Terminal", "terminal")
        .add_item("Web", "wasm");

    if is_macos() {
        select.add_item("macOS Native", "macos");
    }

    if is_windows() {
        select.add_item("Windows Native", "windows");
    }

    select.select_action(0);

    select.on_key(|el, state, event| {
        // Live-sync the highlight into state (run intent only comes from
        // Enter/click at frame level). Lets the args label dim on wasm.
        if event.key == Key::Down || event.key == Key::Up {
            if let Some(idx) = el.get_selected().or_else(|| el.get_focused_index()) {
                if let Some(val) = el.item_value(idx) {
                    state.selected_target = TargetPlatform::parse(&val);
                }
            }
        }
    });
    select
}
