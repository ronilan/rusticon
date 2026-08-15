use crate::state::State;
use crate::ui::{button_package, button_quit, frame, select_options, select_target};
use incredible::*;
use incredible_elements::{App, AppOptions};
use incredible_helpers_layout::arrangers::Arrangers;

pub fn build_app() -> App<State> {
    let app: App<State> = App::new(AppOptions {
        height: Some(8),
        ..Default::default()
    });
    app.exit_combination(Some(KeyCombination::new(Key::Escape, &[])));

    let target_select = select_target::build_select_target();
    let options_select = select_options::build_select_options();
    let button_package = button_package::build_button_package();
    let button_quit = button_quit::build_button_quit();

    // Build frame and add all elements to it
    let frame = frame::build_frame();

    frame.add(target_select);
    frame.add(options_select);
    frame.add(button_quit);
    frame.add(button_package);

    // Add frame-level handlers
    frame
        .on_window(|el, _state, _event| {
            el.elements_to_left();
            el.draw();
        })
        .on_state(|_, state| {
            if state.should_execute {
                exit(0);
            }
        });

    app.add(frame);
    app
}
