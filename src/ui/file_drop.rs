use crate::core::model::{AppPhase, State};
use crate::platform;
use incredible::*;
use incredible_elements::App;
use incredible_elements_extra::{FileDrop, FileDropOptions};

pub fn build() -> FileDrop<State> {
    let file_drop = FileDrop::new(FileDropOptions::default());
    file_drop.width(80).height(24).showed(false).fused(true);

    // Own the drop handling on the element itself (same integration as the
    // incredible-playground FileDrop examples): on each loop tick the hidden
    // FileDrop reports any newly dropped items, and we kick off the import.
    file_drop.on_loop(move |el: &FileDrop<State>, state: &mut State, _event| {
        let Some(items) = el.take_new_items() else {
            return;
        };
        if state.flow.phase == AppPhase::Splash {
            return;
        }
        let Some(item) = items.into_iter().find(|i| !i.is_dir) else {
            return;
        };

        state.flow.launch_start_new = false;
        state.flow.launch_import_started = true;
        state.flow.phase = AppPhase::Splash;
        state.flow.splash_started_ms = None;
        Globals::set_tick_rate(10.0);
        platform::get_io().start_import_drop(item);

        // Redraw the whole app so the splash (and any other phase-dependent
        // screen) becomes visible immediately.
        if let Some(app) = Globals::get_root::<State>()
            .and_then(|root| root.cot::<App<State>>().into_iter().next())
        {
            app.draw();
        }
    });

    file_drop
}
