use std::{collections::HashMap, fs, path::Path, sync::RwLock};

use anyhow::anyhow;
use once_cell::sync::OnceCell;
use which::which;

use crate::types::{config::Config, error::CustomError};

static CONFIG: OnceCell<RwLock<Config>> = OnceCell::new();
static COMMAND_EXISTENCE: OnceCell<RwLock<HashMap<String, bool>>> = OnceCell::new();

pub fn init_config(app_dir: &Path) -> Result<(), CustomError> {
    let config_path = app_dir.join("config.json");

    if !config_path.exists() {
        return Err(CustomError::PathError("Config file not found".to_string()));
    }

    let content = std::fs::read_to_string(&config_path)?;

    let config: Config = serde_json::from_str(&content)?;

    CONFIG
        .set(RwLock::new(config))
        .map_err(|_| CustomError::Anyhow(anyhow!("Config.json initialization failed")))
}

pub fn get_config() -> Result<Config, CustomError> {
    let lock = CONFIG
        .get()
        .ok_or_else(|| CustomError::Anyhow(anyhow!("Config not Initialized".to_string())))?;
    let cfg = lock.read().map_err(|_| {
        CustomError::Anyhow(anyhow!("Failed to acquire read lock on config".to_string()))
    })?;

    Ok(cfg.clone())
}

pub fn update_config(new_cfg: Config, config_path: &Path) -> Result<(), CustomError> {
    let lock = CONFIG
        .get()
        .ok_or_else(|| CustomError::Anyhow(anyhow!("Config not Initialized".to_string())))?;
    {
        let mut cfg = lock.write().map_err(|_| {
            CustomError::Anyhow(anyhow!("Failed to acquire write lock on config".to_string()))
        })?;
        *cfg = new_cfg.clone();
    }
    let json_string = serde_json::to_string_pretty(&new_cfg)?;
    fs::write(config_path, json_string)?;

    Ok(())
}

pub fn init_command_checks() -> Result<(), CustomError> {
    let mut is_commands_available: HashMap<String, bool> = HashMap::new();

    let commands = ["gamemoderun", "mangohud"];

    for command in commands {
        let status = which(command).is_ok();
        is_commands_available.insert(command.to_string(), status);
    }

    COMMAND_EXISTENCE
        .set(RwLock::new(is_commands_available))
        .map_err(|_| CustomError::Anyhow(anyhow!("System commands check init failed")))
}

pub fn command_is_available(command_name: &str) -> Result<bool, CustomError> {
    let lock = COMMAND_EXISTENCE.get().ok_or_else(|| {
        CustomError::Anyhow(anyhow!("System commands check not Initialized".to_string()))
    })?;
    let commands = lock.read().map_err(|_| {
        CustomError::Anyhow(anyhow!(
            "Failed to acquire read lock on system commands check".to_string()
        ))
    })?;

    let result = *commands.get(command_name).unwrap_or(&false);

    Ok(result)
}

#[tauri::command]
pub fn get_command_availability() -> Result<HashMap<String, bool>, CustomError> {
    let lock = COMMAND_EXISTENCE.get().ok_or_else(|| {
        CustomError::Anyhow(anyhow!("System commands check not Initialized".to_string()))
    })?;
    let guard = lock.read().map_err(|_| {
        CustomError::Anyhow(anyhow!(
            "Failed to acquire read lock on system commands check".to_string()
        ))
    })?;

    let result = guard.clone();

    Ok(result)
}
