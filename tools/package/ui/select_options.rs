use incredible::*;
use incredible_elements::{ScrollArea, Select, SelectOptions, Selectable, SelectableItem};

use crate::state::{PackageTarget, State};

pub fn build_select_options() -> Select<State> {
    let select = Select::<State>::new(SelectOptions {
        multi: true,
        ..Default::default()
    });

    select
        .x(34)
        .y(1)
        .width(32)
        .height(6)
        .add_item("Clean before build", "clean")
        .add_item("Bundle app package", "bundle")
        .add_item("Publish package", "publish")
        .add_item("Preview site", "preview")
        .disabled(true);

    select
        .on_key(|el, state, event| {
            // In any of the possible selection keys - update the state
            if event.key == Key::Enter {
                let selected_indices = el.get_selected_indices();
                state.is_clean = selected_indices.contains(&0);
                state.is_publish = selected_indices.contains(&2);
                state.is_bundle = selected_indices.contains(&1) || state.is_publish;
                state.is_preview = selected_indices.contains(&3);
            }
        })
        .on_mouse(|el, state, event| {
            // In any of the possible selection gestures - update the state
            if event.mouse == Mouse::Click {
                let selected_indices = el.get_selected_indices();
                state.is_clean = selected_indices.contains(&0);
                state.is_publish = selected_indices.contains(&2);
                state.is_bundle = selected_indices.contains(&1) || state.is_publish;
                state.is_preview = selected_indices.contains(&3);
            }
        })
        .on_state(|el, state| {
            // No target selected: all items shown (text visible), all disabled.
            el.disabled(state.selected_target.is_none());

            if let Some(fs) = el.elements.cot::<ScrollArea<State>>().first() {
                if let Some(group) = fs.elements.cot::<Selectable<State>>().first() {
                    let items = group.elements.cot::<SelectableItem<State>>();
                    if items.len() >= 4 {
                        let target = state.selected_target.as_ref();

                        let no_target = matches!(target, None);
                        let is_wasm = matches!(target, Some(PackageTarget::Wasm));
                        let is_terminal = matches!(target, Some(PackageTarget::Terminal));

                        // Clean before build
                        items[0].disabled(no_target);

                        // Bundle app package — for non-terminal, non-wasm targets
                        if state.is_publish {
                            items[1].disabled(true);
                            el.select_action(1);
                        } else if !is_terminal && !is_wasm {
                            items[1].disabled(no_target);
                            if state.is_bundle {
                                el.select_action(1);
                            } else {
                                el.unselect_action(1);
                            }
                        } else {
                            items[1].disabled(true);
                            el.unselect_action(1);
                        }

                        // Publish package — not available for wasm
                        items[2].disabled(no_target || is_wasm);

                        // Preview site — wasm only
                        items[3].disabled(!is_wasm);
                    }
                }
            }
        });

    select
}
