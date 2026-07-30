use crate::cargo;
use crate::state::TargetPlatform;
use std::process::{self, Command};

pub fn execute_target(target: TargetPlatform, extra_args: &[String]) {
    let pkg_name = cargo::get_package_name();
    let mut cmd = match target {
        TargetPlatform::Terminal => {
            let mut c = Command::new("cargo");
            c.arg("run").arg("--bin").arg(&pkg_name);
            c
        }
        TargetPlatform::Wasm => {
            let mut c = Command::new("node");
            c.arg("scripts/dev.js");
            c
        }
        TargetPlatform::MacOs => {
            let mut c = Command::new("cargo");
            c.arg("run")
                .arg("--bin")
                .arg(format!("{}_macos", pkg_name))
                .arg("--features")
                .arg("macos-native");
            c
        }
        TargetPlatform::Windows => {
            let mut c = Command::new("cargo");
            c.arg("run")
                .arg("--bin")
                .arg(format!("{}_windows", pkg_name))
                .arg("--features")
                .arg("windows-native");
            c
        }
    };

    if !extra_args.is_empty() {
        cmd.arg("--").args(extra_args);
    }

    let status = cmd.status();
    match status {
        Ok(s) => process::exit(s.code().unwrap_or(0)),
        Err(e) => {
            eprintln!("Failed to execute command: {}", e);
            process::exit(1);
        }
    }
}
