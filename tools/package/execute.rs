use crate::cmd::{build, build_all, bundle, clean, publish, release};
use crate::state::{PackageTarget, State};
use std::path::PathBuf;
use std::process::{self, Command};

pub fn execute_package(state: &State, _extra_args: &[String]) {
    let target = match state.selected_target.as_ref() {
        Some(t) => t,
        None => {
            eprintln!("No target selected. Nothing to do.");
            process::exit(1);
        }
    };

    // Step 1: Clean if requested
    if state.is_clean {
        clean::clean();
    }

    // Step 2: Main action
    let mut produced: Vec<PathBuf> = Vec::new();
    let result = match target {
        PackageTarget::All => {
            if state.is_publish {
                let assets = release::build_release();
                publish::publish(&assets);
            } else {
                build_all::build_all();
            }
            Ok::<(), std::io::Error>(())
        }
        PackageTarget::Terminal => {
            build::terminal();
            if state.is_publish {
                let exe_name = if cfg!(target_os = "windows") {
                    format!("{}.exe", crate::cargo::package_name())
                } else {
                    crate::cargo::package_name()
                };
                if std::path::Path::new("dist/cli").join(&exe_name).exists() {
                    let base = exe_name
                        .strip_suffix(".exe")
                        .unwrap_or(&exe_name)
                        .to_string();
                    let platform = if cfg!(target_os = "macos") {
                        if cfg!(target_arch = "aarch64") {
                            "macos-arm"
                        } else {
                            "macos-intel"
                        }
                    } else if cfg!(target_os = "windows") {
                        "windows"
                    } else {
                        "linux"
                    };
                    let zip_name = format!("{}-terminal-{}.zip", base, platform);
                    release::zip_file("dist/cli", &exe_name, &zip_name);
                    produced.push(std::path::Path::new("dist/cli").join(&zip_name));
                }
            }
            Ok::<(), std::io::Error>(())
        }
        PackageTarget::Wasm => {
            let _ = Command::new("node").arg("scripts/build_web.js").status();
            if state.is_preview {
                let _ = Command::new("npx").args(["vite", "preview"]).status();
            }
            Ok::<(), std::io::Error>(())
        }
        PackageTarget::MacOs => {
            build::macos();
            if state.is_bundle || state.is_publish {
                bundle::macos();
            }
            if state.is_publish {
                // Zip the DMG for publishing (matching CI naming convention)
                if let Ok(files) = std::fs::read_dir("dist/native") {
                    if let Some(dmg_entry) = files
                        .flatten()
                        .find(|f| f.file_name().to_string_lossy().ends_with(".dmg"))
                    {
                        let dmg_name = dmg_entry.file_name().to_string_lossy().to_string();
                        let base = dmg_name
                            .strip_suffix(".dmg")
                            .unwrap_or(&dmg_name)
                            .to_lowercase()
                            .replace(' ', "_");
                        let platform = if cfg!(target_arch = "aarch64") {
                            "macos-arm"
                        } else {
                            "macos-intel"
                        };
                        let zip_name = format!("{}-macos-native-{}.zip", base, platform);
                        release::zip_file("dist/native", &dmg_name, &zip_name);
                        produced.push(std::path::Path::new("dist/native").join(&zip_name));
                    }
                }
            }
            Ok::<(), std::io::Error>(())
        }
        PackageTarget::Windows => {
            build::windows();
            if state.is_bundle || state.is_publish {
                bundle::windows();
            }
            if state.is_publish {
                // Zip the EXE for publishing (matching CI naming convention)
                let app_name = crate::cargo::bundle_app_name();
                let exe_path =
                    std::path::Path::new("dist/native").join(format!("{}.exe", app_name));
                if exe_path.exists() {
                    let base = app_name.to_lowercase().replace(' ', "_");
                    let zip_name = format!("{}-windows-native.zip", base);
                    release::zip_file("dist/native", &format!("{}.exe", app_name), &zip_name);
                    produced.push(std::path::Path::new("dist/native").join(&zip_name));
                }
            }
            Ok::<(), std::io::Error>(())
        }
    };

    // Step 3: Publish if requested (not All - already handled above)
    if state.is_publish && target != &PackageTarget::All {
        publish::publish(&produced);
    }

    match result {
        Ok(_) => process::exit(0),
        Err(e) => {
            eprintln!("Failed to execute package command: {}", e);
            process::exit(1);
        }
    }
}
