use std::path::{Path, PathBuf};

use log::info;

use crate::{types::error::CustomError, utils::fs::create_symlinks_by_filename};

pub fn setup_dxvk(wine_prefix: &str, dxvk_path: &Path) -> Result<(), CustomError> {
    if !dxvk_path.exists() {
        return Err(CustomError::PathError("DXVK path not found".to_string()));
    }

    //setting up dxvk symlinks in system32 and syswow64 directories
    let windows_dir = PathBuf::from(wine_prefix).join("drive_c/windows");

    create_symlinks_by_filename(&dxvk_path.join("x32"), &windows_dir.join("syswow64"), true)?;

    create_symlinks_by_filename(&dxvk_path.join("x64"), &windows_dir.join("system32"), true)?;

    info!("dxvk symlinks setup completed");

    Ok(())
}
