use std::{fs, path::PathBuf};

use anyhow::anyhow;
use log::info;
use std::path::Path;
use tauri::{path::BaseDirectory, AppHandle, Manager};

use crate::{
    types::{
        config::{Config, RunnerVersion, RuntimeVersion},
        error::CustomError,
    },
    utils::{
        fs::extract_archive,
        initialize::{command_is_available, get_config, update_config},
    },
};

fn get_available_wine_directories(
    wine_dir: &Path,
    proton_dir: &Path,
) -> Result<Vec<RunnerVersion>, CustomError> {
    if !wine_dir.is_dir() || !proton_dir.is_dir() {
        return Err(CustomError::PathError(format!(
            "wine/proton directory not found"
        )));
    }

    let mut wine_versions: Vec<RunnerVersion> = Vec::new();

    let mut collect_versions = |dir: &Path, marker: &str| -> Result<(), CustomError> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() && path.join(marker).join("wine").exists() {
                if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                    wine_versions.push(RunnerVersion {
                        name: name.to_string(),
                        path: path.join(marker).display().to_string(),
                    });
                }
            }
        }
        Ok(())
    };

    collect_versions(wine_dir, "bin")?;
    collect_versions(proton_dir, "files/bin")?;

    Ok(wine_versions)
}

fn get_available_dxvk_directories(dxvk_dir: &Path) -> Result<Vec<RuntimeVersion>, CustomError> {
    if !dxvk_dir.exists() || !dxvk_dir.is_dir() {
        return Err(CustomError::PathError(format!("dxvk directory not found")));
    }

    let mut dxvk_versions: Vec<RuntimeVersion> = Vec::new();

    for entry in fs::read_dir(dxvk_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() && path.join("x32").exists() && path.join("x64").exists() {
            if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                dxvk_versions.push(RuntimeVersion {
                    name: name.to_string(),
                    path: path.display().to_string(),
                })
            }
        }
    }

    Ok(dxvk_versions)
}

#[tauri::command]
pub fn populate_settings(app: AppHandle) -> Result<Config, CustomError> {
    let app_dir = app.path().resolve("sbrw-utility", BaseDirectory::Data)?;

    let mut config_string = get_config()?;

    let wine_versions = get_available_wine_directories(
        &app_dir.join("runners/wine"),
        &app_dir.join("runners/proton"),
    )?;
    let dxvk_versions = get_available_dxvk_directories(&app_dir.join("runtime/dxvk"))?;

    config_string.available_wine_versions = Some(wine_versions);
    config_string.available_dxvk_versions = Some(dxvk_versions);

    Ok(config_string)
}

#[tauri::command]
pub fn save_settings(config: Config, app: AppHandle) -> Result<bool, CustomError> {
    let app_dir = app.path().resolve("sbrw-utility", BaseDirectory::Data)?;

    let config_path = app_dir.join("config.json");

    update_config(config, &config_path)?;

    log::info!("config.json updated");

    Ok(true)
}

#[tauri::command]
pub async fn add_wine_version(selected_path: &Path, app: AppHandle) -> Result<(), CustomError> {
    let app_dir = app.path().resolve("sbrw-utility", BaseDirectory::Data)?;

    let runners_dir = app_dir.join("runners");
    let file_name = selected_path
        .file_name()
        .ok_or_else(|| CustomError::PathError("Invalid archive path".to_string()))?
        .to_string_lossy()
        .into_owned();

    let is_wine_archive = file_name.starts_with("wine-");
    let is_proton_archive = file_name.starts_with("Proton-") || file_name.starts_with("GE-Proton");

    let has_valid_extension = file_name.ends_with(".tar.xz") || file_name.ends_with(".tar.gz");

    if !(is_wine_archive || is_proton_archive) || !has_valid_extension {
        info!("extraction exited");
        return Err(CustomError::PathError(format!(
            "Invalid archive file: {}",
            file_name
        )));
    }

    let base_name = file_name
        .trim_end_matches(".tar.xz")
        .trim_end_matches(".tar.gz");

    let target_dir = if is_wine_archive {
        runners_dir.join("wine").join(base_name)
    } else {
        runners_dir.join("proton").join(base_name)
    };

    if target_dir.exists() {
        return Err(CustomError::PathError(format!(
            "Version already exists at {}",
            target_dir.display()
        )));
    }
    fs::create_dir_all(&target_dir)?;

    let selected_path_ = selected_path.to_owned();
    let target_dir_ = target_dir.clone();
    tauri::async_runtime::spawn_blocking(move || extract_archive(&selected_path_, &target_dir_))
        .await
        .map_err(|e| CustomError::Anyhow(anyhow!("Task join error: {}", e)))??;

    info!("Added version from archive: {}", file_name);

    Ok(())
}
