use std::{fs, path::Path};

use crate::{
    setup::{
        dxvk::setup_dxvk, finalize::extract_to_game_dir, resource::download_game_launcher,
        validate::validate_installation, wine::initialize_wine_environment,
    },
    types::error::{CustomError, ValidationError},
    utils::{
        downloader::{build_client, download_using_url},
        fs::{cleanup_file, extract_archive},
    },
};

fn repair_game_launcher(game_dir: &Path) -> Result<(), CustomError> {
    fs::create_dir_all(game_dir)?;

    let launcher_path = download_game_launcher(game_dir)?;
    extract_to_game_dir(game_dir, &launcher_path)?;
    cleanup_file(&launcher_path)?;

    Ok(())
}

fn repair_dxvk(dxvk_version: &str, dxvk_path: &Path, wine_prefix: &str) -> Result<(), CustomError> {
    let modified_version_str = dxvk_version.replace("dxvk-", "v");
    let dxvk_url = format!(
        "https://github.com/doitsujin/dxvk/releases/download/{}/{}.tar.gz",
        modified_version_str, dxvk_version
    );
    let dxvk_file_name = format!("{}.tar.gz", dxvk_version);

    if !dxvk_path.exists() {
        fs::create_dir_all(dxvk_path)?;
        let client = build_client()?;
        let dxvk_archive_path = download_using_url(&client, &dxvk_url, dxvk_path, &dxvk_file_name)?;
        extract_archive(&dxvk_archive_path, dxvk_path)?;
        cleanup_file(&dxvk_archive_path)?;
    }

    setup_dxvk(wine_prefix, dxvk_path)?;

    Ok(())
}

pub fn attempt_repair(
    errors: Vec<ValidationError>,
    wine_prefix: &str,
    wine_path: &Path,
    dxvk_path: &Path,
    dxvk_version: &str,
) -> Result<(), CustomError> {
    let wineboot_path = &wine_path.join("wineboot");
    let wine_prefix_path = Path::new(wine_prefix);
    let game_dir = wine_prefix_path.join("drive_c/SBRW");

    for err in errors {
        match err {
            ValidationError::MissingWinePrefix => {
                initialize_wine_environment(wineboot_path, wine_prefix)?
            }
            ValidationError::MissingGameLauncher => repair_game_launcher(&game_dir)?,
            ValidationError::MissingDXVKSymlinks => {
                repair_dxvk(dxvk_version, dxvk_path, wine_prefix)?
            }
        }
    }

    validate_installation(wine_prefix, wine_path, dxvk_path)?;
    Ok(())
}
