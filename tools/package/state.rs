#[derive(Clone, PartialEq, Debug)]
pub enum PackageTarget {
    All,
    Terminal,
    Wasm,
    MacOs,
    Windows,
}

impl PackageTarget {
    pub fn parse(s: &str) -> Option<Self> {
        match s.trim_start_matches('-').to_lowercase().as_str() {
            "all" | "a" => Some(Self::All),
            "terminal" | "t" => Some(Self::Terminal),
            "wasm" | "web" | "w" => Some(Self::Wasm),
            "macos" | "mac" | "m" => {
                if cfg!(target_os = "macos") {
                    Some(Self::MacOs)
                } else {
                    None
                }
            }
            "windows" | "win" => {
                if cfg!(target_os = "windows") {
                    Some(Self::Windows)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct State {
    pub selected_target: Option<PackageTarget>,
    pub is_clean: bool,
    pub is_preview: bool,
    pub is_bundle: bool,
    pub is_publish: bool,
    pub should_execute: bool,
}
