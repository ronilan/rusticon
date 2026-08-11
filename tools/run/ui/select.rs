use incredible_elements::{FrameKind, Select};

use crate::{
    platform::{is_macos, is_windows},
    state::State,
};

pub fn build_select(select_height: usize) -> Select<State> {
    let select = Select::<State>::default();
    select
        .width(36)
        .height(select_height)
        .frame_kind(Some(FrameKind::Rounded))
        .label("Run")
        .add_item("Terminal", "terminal")
        .add_item("Web", "wasm");

    if is_macos() {
        select.add_item("macOS Native", "macos");
    }

    if is_windows() {
        select.add_item("Windows Native", "windows");
    }

    select.select_action(0);
    select
}
