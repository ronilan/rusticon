use crate::platform::{is_macos, is_windows};

#[derive(Clone, PartialEq, Debug)]
pub enum TargetPlatform {
    Terminal,
    Wasm,
    MacOs,
    Windows,
}

impl TargetPlatform {
    pub fn parse(s: &str) -> Option<Self> {
        match s.trim_start_matches('-').to_lowercase().as_str() {
            "terminal" | "t" => Some(Self::Terminal),
            "wasm" | "web" => Some(Self::Wasm),
            "macos" | "mac" | "m" => {
                if is_macos() {
                    Some(Self::MacOs)
                } else {
                    None
                }
            }
            "windows" | "win" | "w" => {
                if is_windows() {
                    Some(Self::Windows)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

#[derive(Clone, PartialEq, Default)]
pub struct State {
    pub selected_target: Option<TargetPlatform>,
    pub should_run: bool,
}
