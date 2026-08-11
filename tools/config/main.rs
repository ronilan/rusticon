mod cargo;
mod cli;
mod execute;
mod state;
mod ui;

use execute::apply_changes;
use state::State;
use std::process;

fn main() {
    let cli = cli::parse_args();
    let current = cargo::read_current_values();

    let name = cli.name.clone().unwrap_or_else(|| current.name.clone());
    let app_name = cli.app_name.clone().unwrap_or_else(|| {
        if name != current.name {
            name.clone()
        } else {
            current.app_name.clone()
        }
    });
    let tagline = cli
        .tagline
        .clone()
        .unwrap_or_else(|| current.tagline.clone());
    let keywords = cli
        .keywords
        .clone()
        .unwrap_or_else(|| current.keywords.clone());
    let description = cli
        .description
        .clone()
        .unwrap_or_else(|| current.description.clone());

    // All 5 explicitly provided → headless mode
    let all_provided = cli.name.is_some()
        && cli.app_name.is_some()
        && cli.tagline.is_some()
        && cli.keywords.is_some()
        && cli.description.is_some();

    if all_provided {
        let npm_name = name.replace('_', "-");
        apply_changes(
            &name,
            &app_name,
            &npm_name,
            &tagline,
            &keywords,
            &description,
        );
        return;
    }

    // TUI mode
    let init_state = State {
        focused_index: 0,
        name: name.clone(),
        app_name: app_name.clone(),
        tagline: tagline.clone(),
        keywords: keywords.clone(),
        description: description.clone(),
        ..Default::default()
    };
    let (is_valid, rule_text, _) = state::validate(&init_state);
    let state = State {
        is_valid,
        rule_text: rule_text.to_string(),
        rule_invalid: false,
        ..init_state
    };

    let app = ui::app::build_app();
    let final_state = app.run(state).get();

    if final_state.should_apply {
        let final_name = final_state.name;
        let final_app_name = final_state.app_name;
        let final_npm_name = final_name.replace('_', "-");
        let final_tagline = final_state.tagline;
        let final_keywords = final_state.keywords;
        let final_description = final_state.description;

        if final_name.is_empty() {
            eprintln!("Error: Name cannot be empty.");
            process::exit(1);
        }

        apply_changes(
            &final_name,
            &final_app_name,
            &final_npm_name,
            &final_tagline,
            &final_keywords,
            &final_description,
        );
    }
}
