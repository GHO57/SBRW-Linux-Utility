use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

use log::info;
use tauri::AppHandle;

use crate::{
    types::{
        config::{Config, DXVKComponent, GameComponent, KeyValue, WineComponent},
        error::CustomError,
    },
    utils::{
        fs::{cleanup_dir, extract_archive},
        initialize::init_config,
    },
};

pub fn extract_to_game_dir(game_dir: &Path, launcher_path: &Path) -> Result<(), CustomError> {
    //create game directory inside wine's drive_c directory
    fs::create_dir_all(&game_dir)?;

    //extracting the downloaded game launcher
    extract_archive(&launcher_path, &game_dir)?;

    info!("Extracted game launcher to drive_c/SBRW");

    Ok(())
}

fn copy_machine_id_to_prefix(machine_id_path: &Path) -> Result<(), CustomError> {
    fs::create_dir_all(machine_id_path)?;

    let cmd_output = Command::new("cat").arg("/etc/machine-id").output()?;

    let mut machine_id_file = fs::File::create(machine_id_path.join("machine-id"))?;

    machine_id_file.write_all(&cmd_output.stdout)?;

    info!("Machine id copied");

    Ok(())
}

fn write_config(
    wine_prefix: &str,
    game_dir: &Path,
    app_dir: &Path,
    wine_version: &str,
    wine_path: &Path,
    dxvk_version: &str,
    dxvk_path: &Path,
) -> Result<(), CustomError> {
    //Creating config.json file
    let wine = WineComponent {
        version: wine_version.to_string(),
        path: wine_path.display().to_string(),
        esync: true,
        fsync: true,
        feral_gamemode: true,
    };

    let dxvk = DXVKComponent {
        version: dxvk_version.to_string(),
        path: dxvk_path.display().to_string(),
        enabled: true,
    };

    let environment_variables: Vec<KeyValue> = Vec::new();
    let mut dll_overrides: Vec<KeyValue> = Vec::new();
    dll_overrides.push(KeyValue {
        key: "dinput8".to_string(),
        value: "n,b".to_string(),
    });

    let game = GameComponent {
        prefix: wine_prefix.to_string(),
        directory: game_dir.display().to_string(),
        launcher: game_dir.join("GameLauncher.exe").display().to_string(),
        mangohud: false,
        environment_variables: environment_variables,
        dll_overrides: dll_overrides,
    };

    let config = Config::new(wine, dxvk, game);

    let json_string = serde_json::to_string_pretty(&config)?;

    fs::write(app_dir.join("config.json"), json_string)?;

    info!("config.json created");

    Ok(())
}

fn cleanup_setup(app_dir: &Path) -> Result<(), CustomError> {
    let tools_dir = app_dir.join("tools");

    //cleaning up tools folder
    cleanup_dir(&tools_dir, "tools")?;

    Ok(())
}

pub fn finalize_setup(
    wine_prefix: &str,
    launcher_path: &Path,
    app_dir: &Path,
    wine_version: &str,
    wine_path: &Path,
    dxvk_version: &str,
    dxvk_path: &Path,
) -> Result<(), CustomError> {
    let wine_prefix_path = PathBuf::from(wine_prefix);
    let game_dir = wine_prefix_path.join("drive_c/SBRW");

    extract_to_game_dir(&game_dir, &launcher_path)?;

    copy_machine_id_to_prefix(&wine_prefix_path.join("drive_c/etc"))?;

    write_config(
        wine_prefix,
        &game_dir,
        app_dir,
        wine_version,
        wine_path,
        dxvk_version,
        dxvk_path,
    )?;

    init_config(app_dir)?;

    cleanup_setup(app_dir)?;

    Ok(())
}
