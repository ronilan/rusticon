mod cargo;
mod cli;
mod cmd;
mod execute;
mod platform;
mod state;
mod ui;

use execute::execute_package;
use state::State;

fn main() {
    let (state, extra_args) = cli::parse_args();

    // Headless mode: target specified via argument
    if state.selected_target.is_some() {
        execute_package(&state, &extra_args);
        return;
    }

    // UI mode
    let app = ui::app::build_app();

    let final_state = app.run(State::default()).get();

    if final_state.should_execute {
        execute_package(&final_state, &extra_args);
    }
}
