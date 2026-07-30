use crate::cargo;
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn terminal() {
    let name = cargo::package_name();
    println!("Building terminal binary: {}", name);

    let status = Command::new("cargo")
        .args(["build", "--release", "--bin", &name])
        .status()
        .expect("Failed to run cargo build");

    if !status.success() {
        eprintln!("Cargo build failed");
        std::process::exit(1);
    }

    let exe_name = if cfg!(target_os = "windows") {
        format!("{}.exe", name)
    } else {
        name.clone()
    };

    fs::create_dir_all("dist/cli").expect("Failed to create dist/cli");
    fs::copy(
        Path::new("target/release").join(&exe_name),
        Path::new("dist/cli").join(&exe_name),
    )
    .expect("Failed to copy binary");

    println!("Binary copied to dist/cli/{}", exe_name);
}

pub fn macos() {
    let name = cargo::package_name();
    let app_name = cargo::bundle_app_name();
    let bin_name = format!("{}_macos", name);

    println!("Building macOS native: {}", app_name);
    // SAFETY: Setting environment variable before spawning child process;
    // no other threads are reading it at this point.
    unsafe {
        std::env::set_var("APP_NAME", &app_name);
    }

    let status = Command::new("cargo")
        .args([
            "build",
            "--release",
            "--bin",
            &bin_name,
            "--features",
            "macos-native",
        ])
        .status()
        .expect("Failed to run cargo build");

    if !status.success() {
        eprintln!("Cargo build failed");
        std::process::exit(1);
    }

    fs::create_dir_all("dist/native").expect("Failed to create dist/native");
    fs::copy(
        Path::new("target/release").join(&bin_name),
        Path::new("dist/native").join(&bin_name),
    )
    .expect("Failed to copy binary");

    println!("Binary copied to dist/native/{}", bin_name);
}

pub fn windows() {
    let name = cargo::package_name();
    let app_name = cargo::bundle_app_name();
    let bin_name = format!("{}_windows", name);

    println!("Building Windows native: {}", app_name);
    // SAFETY: Setting environment variable before spawning child process;
    // no other threads are reading it at this point.
    unsafe {
        std::env::set_var("APP_NAME", &app_name);
    }

    let status = Command::new("cargo")
        .args([
            "build",
            "--release",
            "--bin",
            &bin_name,
            "--features",
            "windows-native",
        ])
        .status()
        .expect("Failed to run cargo build");

    if !status.success() {
        eprintln!("Cargo build failed");
        std::process::exit(1);
    }

    fs::create_dir_all("dist/native").expect("Failed to create dist/native");
    let exe_name = format!("{}.exe", app_name);
    fs::copy(
        Path::new("target/release").join(format!("{}.exe", bin_name)),
        Path::new("dist/native").join(&exe_name),
    )
    .expect("Failed to copy binary");

    println!("Binary copied to dist/native/{}", exe_name);
}
