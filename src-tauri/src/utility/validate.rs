use std::path::Path;

use log::warn;
use which::which;

use crate::{
    setup::{
        check_vc_runtimes,
        validate::{
            check_dxvk_symlinks, check_game_launcher_exe, check_wine_bin_dir, check_wine_prefix,
        },
    },
    types::error::{CustomError, ValidationError},
    utility::repair::attempt_repair,
};

pub fn validate_on_startup(
    wine_prefix: &str,
    wine_path: &Path,
    dxvk_path: &Path,
    dxvk_version: &str,
) -> Result<(), CustomError> {
    let mut errors: Vec<ValidationError> = Vec::new();

    //check wine prefix' registry files existence
    let wine_prefix_path = Path::new(wine_prefix);
    if let Err(_) = check_wine_prefix(&wine_prefix_path) {
        warn!("Validation Error: wine prefix is missing, attempting repair");
        errors.push(ValidationError::MissingWinePrefix);
    }

    //check GameLauncher.exe file existence
    if let Err(_) = check_game_launcher_exe(&wine_prefix_path) {
        warn!("Validation Error: Game Launcher is missing, attempting repair");
        errors.push(ValidationError::MissingGameLauncher);
    }

    //check wine bin existence
    if let Err(_) = check_wine_bin_dir(&wine_path) {
        warn!("Validation Error: wine is missing");
        // errors.push(ValidationError::MissingWineBinDir);
    }

    //check dxvk symlinks
    if let Err(_) = check_dxvk_symlinks(&dxvk_path, &wine_prefix_path) {
        warn!("Validation Error: dxvk symlinks are missing, attempting repair");
        errors.push(ValidationError::MissingDXVKSymlinks);
    }

    //check vc runtimes
    if let Err(_) = check_vc_runtimes(&wine_prefix_path) {
        warn!("Validation Error: VC runtimes are missing");
        // errors.push(ValidationError::MissingVCRuntimes);
    }

    if !errors.is_empty() {
        attempt_repair(errors, wine_prefix, wine_path, dxvk_path, dxvk_version)?;
    }

    Ok(())
}
