mod cargo;
mod cli;
mod execute;
mod platform;
mod state;
mod ui;

use execute::execute_target;
use state::State;

fn main() {
    let (target, extra_args) = cli::parse_args();

    // Headless mode: valid target argument was passed
    if let Some(target) = target {
        execute_target(target, &extra_args);
        return;
    }

    // UI mode
    let app = ui::app::build_app();
    let final_state = app.run(State::default()).get();

    if let Some(selected_target) = final_state.selected_target {
        execute_target(selected_target, &extra_args);
    }
}
