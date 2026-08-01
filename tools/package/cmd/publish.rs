use std::path::PathBuf;
use std::process::{Command, Output};

fn run_gh(args: &[&str]) -> Result<Output, String> {
    let output = Command::new("gh")
        .args(args)
        .output()
        .map_err(|e| format!("Failed to run gh {}: {}", args.join(" "), e))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("gh {} failed: {}", args.join(" "), stderr.trim()));
    }
    Ok(output)
}

fn sleep(ms: u64) {
    #[cfg(target_os = "windows")]
    {
        let _ = Command::new("powershell")
            .args(["-Command", &format!("Start-Sleep -Milliseconds {}", ms)])
            .status();
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = Command::new("sleep")
            .arg(format!("{}.{}", ms / 1000, ms % 1000))
            .status();
    }
}

fn check_gh() -> bool {
    Command::new("gh")
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn publish(zips: &[PathBuf]) {
    println!("Publishing to \"latest\" release via gh CLI");

    if !check_gh() {
        eprintln!(
            "Error: 'gh' CLI not found. Install GitHub CLI (https://cli.github.com) and authenticate with 'gh auth login'."
        );
        return;
    }

    if zips.is_empty() {
        println!("No zip artifacts to publish. Nothing to do.");
        return;
    }

    // Ensure release exists
    let exists = run_gh(&["release", "view", "latest"]).is_ok();

    if !exists {
        println!("Creating \"latest\" release...");
        if let Err(e) = run_gh(&[
            "release",
            "create",
            "latest",
            "--title",
            "Latest Build",
            "--prerelease",
            "--notes",
            "Automated local build artifacts.",
        ]) {
            eprintln!("Error creating release: {}", e);
            return;
        }
    }

    // Read existing assets
    let assets_output = match run_gh(&["release", "view", "latest", "--json", "assets"]) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("Failed to get release assets: {}", e);
            return;
        }
    };

    let assets_json: serde_json::Value =
        serde_json::from_slice(&assets_output.stdout).unwrap_or(serde_json::Value::Null);

    let existing_assets: Vec<String> = assets_json
        .get("assets")
        .and_then(|a| a.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|a| {
                    a.get("name")
                        .and_then(|n| n.as_str().map(|s| s.to_lowercase()))
                })
                .collect()
        })
        .unwrap_or_default();

    for path in zips {
        let file_name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let file_name_lower = file_name.to_lowercase();

        if existing_assets.contains(&file_name_lower) {
            println!("Removing existing asset \"{}\"", file_name);

            if let Err(e) = run_gh(&["release", "delete-asset", "latest", &file_name, "-y"]) {
                eprintln!("Error removing asset: {}", e);
            }

            // Wait for asset to be deleted
            loop {
                let check = run_gh(&["release", "view", "latest", "--json", "assets"]);

                if let Ok(output) = check {
                    let json: serde_json::Value =
                        serde_json::from_slice(&output.stdout).unwrap_or(serde_json::Value::Null);
                    let still_exists = json
                        .get("assets")
                        .and_then(|a| a.as_array())
                        .map(|arr| {
                            arr.iter().any(|a| {
                                a.get("name")
                                    .and_then(|n| n.as_str())
                                    .map(|n| n.to_lowercase() == file_name_lower)
                                    .unwrap_or(false)
                            })
                        })
                        .unwrap_or(true);
                    if !still_exists {
                        break;
                    }
                }
                sleep(500);
            }
        }

        println!("Uploading \"{}\"", file_name);
        if let Err(e) = run_gh(&["release", "upload", "latest", &path.to_string_lossy()]) {
            eprintln!("Error uploading {}: {}", file_name, e);
        }
    }

    println!("Done publishing");
}
