use crate::state::TargetPlatform;
use std::env;

pub fn parse_args() -> (Option<TargetPlatform>, Vec<String>) {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut target: Option<TargetPlatform> = None;
    let mut extra_args: Vec<String> = Vec::new();

    if !args.is_empty() {
        target = TargetPlatform::parse(&args[0]);
        if target.is_some() {
            extra_args = args[1..].to_vec();
        }
    }

    (target, extra_args)
}
