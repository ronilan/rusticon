
mod app;
mod platform;
mod runtime;
mod core;
mod features;
mod screens;
mod ui;


#[cfg(all(not(target_arch = "wasm32"), not(feature = "macos-native")))]
fn main() {
    platform::init();
    runtime::run();
}

#[cfg(target_arch = "wasm32")]
fn main() {}
