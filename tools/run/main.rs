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

    // UI mode: prefill the args label with any orphan CLI args
    // (e.g. `./run foo.txt` → pick Terminal → forwards `foo.txt`).
    let init_state = State {
        args_text: cli::join_args(&extra_args),
        ..Default::default()
    };
    let app = ui::app::build_app();
    let final_state = app.run(init_state).get();

    // Only execute on explicit run (Enter / click-to-run). Escape quits
    // the menu, and navigation merely updates selected_target — neither
    // must launch anything.
    if final_state.should_run {
        if let Some(selected_target) = final_state.selected_target {
            let args = cli::split_args(&final_state.args_text);
            execute_target(selected_target, &args);
        }
    }
}
