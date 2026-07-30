use crate::state::{PackageTarget, State};
use std::env;

pub fn parse_args() -> (State, Vec<String>) {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut state = State::default();
    let mut extra_args: Vec<String> = Vec::new();

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--clean" => state.is_clean = true,
            "--bundle" => state.is_bundle = true,
            "--preview" => state.is_preview = true,
            "--publish" => state.is_publish = true,
            arg => {
                if state.selected_target.is_none() {
                    if let Some(t) = PackageTarget::parse(arg) {
                        state.selected_target = Some(t);
                    } else {
                        extra_args.push(arg.to_string());
                    }
                } else {
                    extra_args.push(arg.to_string());
                }
            }
        }
        i += 1;
    }

    (state, extra_args)
}
