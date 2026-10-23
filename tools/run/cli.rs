use crate::state::TargetPlatform;
use std::env;

pub fn parse_args() -> (Option<TargetPlatform>, Vec<String>) {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        return (None, Vec::new());
    }

    match TargetPlatform::parse(&args[0]) {
        Some(target) => {
            let mut rest = &args[1..];
            // Allow an explicit `--` separator: `./run terminal -- foo.txt`
            if rest.first().map(|s| s.as_str() == "--").unwrap_or(false) {
                rest = &rest[1..];
            }
            (Some(target), rest.to_vec())
        }
        // No valid target → TUI mode. Keep all args so the menu can
        // prefill them (e.g. `./run foo.txt` → pick Terminal → forwards foo.txt).
        None => {
            let rest = if args.first().map(|s| s.as_str() == "--").unwrap_or(false) {
                args[1..].to_vec()
            } else {
                args
            };
            (None, rest)
        }
    }
}

/// Splits freeform args text (from the TUI input or CLI prefill) into
/// individual arguments, respecting single/double quotes and `\` escapes.
pub fn split_args(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut has = false;
    let mut chars = s.chars().peekable();
    let mut in_single = false;
    let mut in_double = false;

    while let Some(c) = chars.next() {
        if in_single {
            if c == '\'' {
                in_single = false;
            } else {
                cur.push(c);
            }
        } else if in_double {
            if c == '"' {
                in_double = false;
            } else if c == '\\' {
                match chars.next() {
                    Some(n) => match n {
                        '"' | '\\' | '$' | '`' => cur.push(n),
                        _ => {
                            cur.push('\\');
                            cur.push(n);
                        }
                    },
                    None => cur.push('\\'),
                }
            } else {
                cur.push(c);
            }
        } else if c == '\'' {
            in_single = true;
            has = true;
        } else if c == '"' {
            in_double = true;
            has = true;
        } else if c == '\\' {
            if let Some(n) = chars.next() {
                cur.push(n);
            }
            has = true;
        } else if c.is_whitespace() {
            if has || !cur.is_empty() {
                out.push(std::mem::take(&mut cur));
                has = false;
            }
        } else {
            cur.push(c);
            has = true;
        }
    }

    if has || !cur.is_empty() {
        out.push(cur);
    }
    out
}

/// Joins args for display/prefill in the TUI input, quoting as needed so
/// `split_args(join_args(args))` round-trips.
pub fn join_args(args: &[String]) -> String {
    args.iter()
        .map(|a| {
            if a.is_empty() {
                "\"\"".to_string()
            } else if a
                .chars()
                .any(|c| c.is_whitespace() || c == '"' || c == '\'' || c == '\\')
            {
                format!("\"{}\"", a.replace('\\', "\\\\").replace('"', "\\\""))
            } else {
                a.clone()
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
