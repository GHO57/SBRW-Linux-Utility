mod setup;
mod types;
mod utility;
mod utils;

use log::info;
use tauri::path::BaseDirectory;
use tauri::Manager;
use tauri_plugin_log;

use crate::setup::environment::setup_environment;
use crate::utility::launcher::check_config;
use crate::utility::launcher::launch_game;
use crate::utility::launcher::stop_game;
use crate::utility::setting::add_wine_version;
use crate::utility::setting::populate_settings;
use crate::utility::setting::save_settings;
use crate::utils::initialize::get_command_availability;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_handle = app.handle();

            let app_dir = app_handle
                .path()
                .resolve("sbrw-utility", BaseDirectory::Data)?;

            if let Err(config_err) = crate::utils::initialize::init_config(&app_dir) {
                log::error!("Config init failed: {}", config_err);
            }

            // info!("Config file initialized");

            if let Err(command_checks_err) = crate::utils::initialize::init_command_checks() {
                log::error!("Command checks init failed: {}", command_checks_err);
            }

            // info!("Command checks initialized");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            setup_environment,
            check_config,
            launch_game,
            stop_game,
            populate_settings,
            save_settings,
            get_command_availability,
            add_wine_version
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
