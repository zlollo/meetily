#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use log;

fn main() {
    std::env::set_var("RUST_LOG", "info");

    // On macOS, tauri-plugin-log owns the logger so Finder-launched startup
    // failures are persisted under ~/Library/Logs/<bundle identifier>/.
    // Keep env_logger for terminal-oriented development on other platforms.
    #[cfg(not(target_os = "macos"))]
    env_logger::init();

    log::info!("Starting application...");
    app_lib::run();
}
