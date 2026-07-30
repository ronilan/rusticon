use crate::cmd::{build, bundle};
use std::process::Command;

pub fn build_all() {
    // cargo update
    let _ = Command::new("cargo").arg("update").status();

    // Terminal (Rust)
    build::terminal();

    // Web (JS-dependent - keep as node call)
    let _ = Command::new("node").arg("scripts/build_web.js").status();

    // Platform-specific
    #[cfg(target_os = "macos")]
    {
        build::macos();
        bundle::macos();
    }

    #[cfg(target_os = "windows")]
    {
        build::windows();
        bundle::windows();
    }
}
