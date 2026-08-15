use incredible::*;
use incredible_elements::{App, AppOptions, Select};
use incredible_helpers_layout::arrangers::Arrangers;

use crate::{
    platform::{is_macos, is_windows},
    state::{State, TargetPlatform},
    ui::select,
};

pub fn build_app() -> App<State> {
    // UI mode: calculate heights dynamically based on item count
    // TODO: get this out of app
    let mut item_count = 2usize;
    if is_macos() || is_windows() {
        item_count += 1;
    }

    let select_height = item_count + 2; // items + top/bottom rounded border
    let app_height = select_height + 1;

    let app: App<State> = App::new(AppOptions {
        height: Some(app_height),
        ..Default::default()
    });
    app.exit_combination(Some(KeyCombination::new(Key::Escape, &[])));

    let select = select::build_select(select_height);
    app.add(select);

    app.on_key(|el, state, event| {
        if event.key == Key::Enter {
            if let Some(select) = el.elements.cot::<Select<State>>().first() {
                let idx = select.get_selected().or_else(|| select.get_focused_index());
                if let Some(idx) = idx {
                    if let Some(val) = select.item_value(idx) {
                        state.selected_target = TargetPlatform::parse(&val);
                        state.should_run = true;
                    }
                }
            }
        }
    })
    .on_mouse(|el, state, event| {
        if event.mouse == Mouse::Click {
            if let Some(select) = el.elements.cot::<Select<State>>().first() {
                let idx = select.get_selected().or_else(|| select.get_focused_index());
                if let Some(idx) = idx {
                    if let Some(val) = select.item_value(idx) {
                        state.selected_target = TargetPlatform::parse(&val);
                        state.should_run = true;
                    }
                }
            }
        }
    })
    .on_window(|el, _state, _event| {
        el.elements_to_left();
        el.draw();
    })
    .on_state(|_, state| {
        if state.should_run {
            exit(0);
        }
    });

    app
}
