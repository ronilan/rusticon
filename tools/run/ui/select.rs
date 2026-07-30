use incredible_elements::{FrameKind, Select};

use crate::state::State;

pub fn build_select(select_height: usize) -> Select<State> {
    let select = Select::<State>::default();
    select
        .width(36)
        .height(select_height)
        .frame_kind(Some(FrameKind::Rounded))
        .add_item("Terminal", "terminal")
        .add_item("Web", "wasm");

    #[cfg(target_os = "macos")]
    select.add_item("macOS Native", "macos");

    #[cfg(target_os = "windows")]
    select.add_item("Windows Native", "windows");

    select.select_action(0);
    select
}
