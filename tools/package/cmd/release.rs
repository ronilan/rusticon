use std::fs;
use std::process::{Command, Stdio};

use crate::cmd::build;

pub fn zip_file(dir: &str, filename: &str, zip_name: &str) {
    let zip_path = std::env::current_dir()
        .expect("Failed to get current directory")
        .join(dir)
        .join(zip_name);

    if cfg!(target_os = "windows") {
        let _ = Command::new("powershell")
            .args([
                "-Command",
                &format!(
                    "Compress-Archive -Path '{}' -DestinationPath '{}'",
                    filename,
                    zip_path.to_string_lossy()
                ),
            ])
            .current_dir(dir)
            .status();
    } else {
        let _ = Command::new("zip")
            .stdout(Stdio::null())
            .args(["-r", &zip_path.to_string_lossy(), filename])
            .current_dir(dir)
            .status();
    }

    println!("Zipped: {:?}", zip_path);
}

pub fn build_release() {
    let mut release_assets: Vec<String> = Vec::new();

    // Clean
    fs::remove_dir_all("dist").ok();

    // Terminal
    build::terminal();

    // Find terminal binary
    if let Ok(files) = fs::read_dir("dist/cli") {
        for file in files.flatten() {
            let exe = file.file_name().to_string_lossy().to_string();
            let base = exe.strip_suffix(".exe").unwrap_or(&exe).to_string();
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
            zip_file("dist/cli", &exe, &zip_name);
            release_assets.push(zip_name);
            break;
        }
    }

    // macOS native
    #[cfg(target_os = "macos")]
    {
        build::macos();
        crate::cmd::bundle::macos();

        if let Ok(files) = fs::read_dir("dist/native") {
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
                zip_file("dist/native", &dmg_name, &zip_name);
                release_assets.push(zip_name);
            }
        }
    }

    // Windows native
    #[cfg(target_os = "windows")]
    {
        build::windows();

        let app_name = crate::cargo::bundle_app_name();
        let original_exe = std::path::Path::new("dist/native").join(format!("{}.exe", app_name));
        if original_exe.exists() {
            let base = app_name.to_lowercase().replace(' ', "_");
            let zip_name = format!("{}-windows-native.zip", base);
            zip_file("dist/native", &format!("{}.exe", app_name), &zip_name);
            release_assets.push(zip_name);
        }
    }

    println!("Done. Produced:");
    for asset in &release_assets {
        println!("  {}", asset);
    }
}
