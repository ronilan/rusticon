mod app;
mod core;
mod features;
mod platform;
mod runtime;
mod screens;
mod ui;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    platform::init();
    runtime::run();
}

#[cfg(target_arch = "wasm32")]
fn main() {}
