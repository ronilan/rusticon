use incredible::*;
use incredible_elements::{App, AppOptions};

use crate::state::State;
use crate::ui::{
    button_apply, button_quit, frame, input_app_name, input_description, input_keywords,
    input_name, input_tagline, label_help, label_rule,
};

pub fn build_app() -> App<State> {
    let app: App<State> = App::new(AppOptions {
        height: Some(24),
        ..Default::default()
    });
    app.exit_combination(Some(KeyCombination::new(Key::Escape, &[])));

    app.on_state(|_, state| {
        if state.should_apply || state.should_quit {
            exit(0);
        }
    });

    // ----- Frame -----
    let frame = frame::build_frame();

    frame.add(input_app_name::build_input_app_name());
    frame.add(input_name::build_input_name());
    frame.add(input_tagline::build_input_tagline());
    frame.add(input_keywords::build_input_keywords());
    frame.add(input_description::build_input_description());
    frame.add(button_quit::build_button_quit());
    frame.add(button_apply::build_button_apply());
    frame.add(label_help::build_label_help());
    frame.add(label_rule::build_label_rule());
    app.add(frame);

    app
}
