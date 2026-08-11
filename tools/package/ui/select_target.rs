use incredible::*;
use incredible_elements::{Select, SelectOptions};

use crate::{
    platform::{is_macos, is_windows},
    state::{PackageTarget, State},
};

pub fn build_select_target() -> Select<State> {
    let select = Select::<State>::new(SelectOptions {
        allow_unselect: true,
        ..Default::default()
    });
    select
        .x(1)
        .y(1)
        .width(32)
        .height(6)
        .focused(true)
        .add_item("All Targets", "all")
        .add_item("Terminal", "terminal")
        .add_item("Web", "wasm");

    if is_macos() {
        select.add_item("macOS Native", "macos");
    }

    if is_windows() {
        select.add_item("Windows Native", "windows");
    }

    select
        .on_key(|el, state, event| {
            // In any of the possible selection keys - update the state
            if event.key == Key::Down || event.key == Key::Up || event.key == Key::Enter {
                if let Some(idx) = el.get_selected() {
                    if let Some(val) = el.item_value(idx) {
                        state.selected_target = PackageTarget::parse(&val);
                    }
                } else {
                    state.selected_target = None;
                }
            }
        })
        .on_mouse(|el, state, event| {
            // In any of the possible selection gestures - update the state
            if event.mouse == Mouse::Click {
                if let Some(idx) = el.get_selected() {
                    if let Some(val) = el.item_value(idx) {
                        state.selected_target = PackageTarget::parse(&val);
                    }
                } else {
                    state.selected_target = None;
                }
            }
        });

    select
}
