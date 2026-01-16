use std::{
    path::{Path, PathBuf},
    process::Stdio,
};

use anyhow::anyhow;

use log::{error, info};
use tauri::{path::BaseDirectory, AppHandle, Manager, Window};
use which::which;

use crate::{
    types::error::CustomError,
    utility::validate::validate_on_startup,
    utils::{
        emitter::report_event,
        initialize::{command_is_available, get_config},
    },
};

#[tauri::command]
pub async fn check_config() -> Result<bool, CustomError> {
    let config = match get_config() {
        Ok(cfg) => cfg,
        Err(_) => return Ok(false),
    };

    let wine_prefix = config.game.prefix;
    let wine_path = PathBuf::from(config.wine.path);
    let dxvk_path = PathBuf::from(config.dxvk.path);
    let dxvk_version = config.dxvk.version;

    let result = tauri::async_runtime::spawn_blocking(move || {
        validate_on_startup(&wine_prefix, &wine_path, &dxvk_path, &dxvk_version)
    })
    .await
    .map_err(|e| CustomError::Anyhow(anyhow!("Task join error: {}", e)))?;

    Ok(result.is_ok())
}

#[tauri::command]
pub fn launch_game(window: Window) -> Result<(), CustomError> {
    report_event(&window, "game-state", "GameLaunching")?;

    let config = get_config()?;

    let mut default_wine_overrides: Vec<String> = vec!["winemenubuilder=".to_string()];

    let wine_cmd_path = Path::new(&config.wine.path).join("wine");
    let wine_cmd_str = wine_cmd_path
        .to_str()
        .ok_or_else(|| CustomError::Anyhow(anyhow!("Cannot convert wine path to string")))?;
    let dxvk_enabled = config.dxvk.enabled;
    let game_dir = config.game.directory;
    let launcher_path = config.game.launcher;
    let feral_gamemode = config.wine.feral_gamemode;
    let is_feral_gamemode_available = command_is_available("gamemoderun")?;
    let mangohud = config.game.mangohud;
    let is_mangohud_available = command_is_available("mangohud")?;
    let esync = if config.wine.esync { "1" } else { "0" };
    let fsync = if config.wine.fsync { "1" } else { "0" };
    let wine_prefix = config.game.prefix;
    let provided_overrides = config.game.dll_overrides;
    let environment_variables = config.game.environment_variables;

    let mut wine_dll_overrides: Vec<String> = Vec::new();

    wine_dll_overrides.push(
        if dxvk_enabled {
            "d3d9,d3d10,d3d10_1,d3d10core,d3d11,dxgi=n"
        } else {
            "d3d9,d3d10,d3d10_1,d3d10core,d3d11,dxgi=b"
        }
        .to_string(),
    );

    for override_var in provided_overrides {
        wine_dll_overrides.push(format!("{}={}", override_var.key, override_var.value));
    }

    wine_dll_overrides.append(&mut default_wine_overrides);

    let wine_dll_overrides_str = wine_dll_overrides.join(";");

    let command = if is_mangohud_available && mangohud {
        "mangohud"
    } else if feral_gamemode && is_feral_gamemode_available {
        "gamemoderun"
    } else {
        wine_cmd_str
    };

    let mut arguments: Vec<String> = Vec::new();
    if is_mangohud_available && mangohud {
        arguments.push("gamemoderun".to_string());
        arguments.push(wine_cmd_str.to_string());
    } else if feral_gamemode && is_feral_gamemode_available {
        arguments.push(wine_cmd_str.to_string());
    }
    arguments.push(launcher_path);

    let mut envs: Vec<(String, String)> = Vec::new();

    for env in environment_variables {
        envs.push((env.key, env.value));
    }

    envs.push(("WINEPREFIX".to_string(), wine_prefix.to_string()));
    envs.push(("WINEESYNC".to_string(), esync.to_string()));
    envs.push(("WINEFSYNC".to_string(), fsync.to_string()));
    envs.push(("WINEDLLOVERRIDES".to_string(), wine_dll_overrides_str));

    info!("Launching game");

    // Log the command
    println!("Running command: {}", command);
    println!("Arguments: {:?}", arguments);
    println!("Environment variables:");
    for (key, value) in &envs {
        println!("{}={}", key, value);
    }

    let mut proc = std::process::Command::new(command)
        .args(arguments)
        .envs(envs)
        .current_dir(game_dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|e| {
            let _ = report_event(&window, "game-state", "GameStopped");
            info!("Failed to start command: {}", e);
            CustomError::Anyhow(anyhow!("Failed to start command: {}", e))
        })?;

    let window_ = window.clone();
    let wine_cmd_ = config.wine.path.clone();
    let wine_prefix_ = wine_prefix.clone();

    std::thread::spawn(move || {
        let result = (|| -> Result<(), CustomError> {
            //waiting until proc exits so that it doesn't leave zombie process behind
            let _ = proc.wait()?;
            report_event(&window, "game-state", "GameRunning")?;
            info!("Game is running");

            let wineserver_wait_status =
                std::process::Command::new(Path::new(&wine_cmd_).join("wineserver"))
                    .arg("-w")
                    .env("WINEPREFIX", wine_prefix_)
                    .status()?;

            report_event(&window_, "game-state", "GameStopped")?;
            info!("Game exited with: {:?}", wineserver_wait_status);

            Ok(())
        })();

        if let Err(err) = result {
            error!("wineserver watcher failed: {:?}", err);
        }
    });

    Ok(())
}

#[tauri::command]
pub fn stop_game() -> Result<(), CustomError> {
    let config = get_config()?;

    let wineserver_cmd = Path::new(&config.wine.path).join("wineserver");
    let wine_prefix = config.game.prefix;

    std::process::Command::new(wineserver_cmd)
        .arg("-k")
        .env("WINEPREFIX", wine_prefix)
        .status()?;

    Ok(())
}
