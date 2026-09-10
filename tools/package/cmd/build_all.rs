use crate::cmd::{build, bundle, web};
use std::process::Command;

pub fn build_all() {
    // cargo update
    let _ = Command::new("cargo").arg("update").status();

    // Terminal (Rust)
    build::terminal();

    // Web (Rust)
    let _ = web::build();

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
