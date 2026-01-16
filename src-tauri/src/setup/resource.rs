use std::{
    fs,
    path::{Path, PathBuf},
};

use log::info;

use crate::{
    types::error::CustomError,
    utils::{
        downloader::{build_client, download_using_url, get_latest_download_url},
        fs::extract_archive,
    },
};

pub fn download_game_launcher(dest: &Path) -> Result<PathBuf, CustomError> {
    fs::create_dir_all(dest)?;

    let client = build_client()?;

    let game_launcher_repo_url =
        "https://api.github.com/repos/SoapboxRaceWorld/GameLauncher_NFSW/releases/latest";
    let game_launcher_url =
        get_latest_download_url(&client, game_launcher_repo_url, "Unix.Release")?;

    // Extract file name from URL
    let file_name = Path::new(&game_launcher_url)
        .file_name()
        .and_then(|os_str| os_str.to_str())
        .ok_or_else(|| CustomError::PathError("Invalid file name in URL".to_string()))?;

    //downloading the game launcher
    info!("Downloading SBRW launcher");
    let launcher_path = download_using_url(&client, &game_launcher_url, dest, file_name)?;

    Ok(launcher_path)
}

pub fn write_bundled_resources(tools_dir: &Path) -> Result<(), CustomError> {
    fs::create_dir_all(tools_dir)?;

    //copying bundled Wine and DXVK to /.local/share/sbrw-utility/tools
    info!("Writing bundled resources to tools folder");
    let wine_bytes = include_bytes!("../../resources/wine-10.8-staging-amd64.tar.xz");
    let dxvk_bytes = include_bytes!("../../resources/dxvk-1.10.3.tar.gz");
    let wine_mono_bytes = include_bytes!("../../resources/wine-mono-10.0.0-x86.msi");
    let vc_redist_x86_bytes = include_bytes!("../../resources/VC_redist.x86.exe");
    let vc_redist_x64_bytes = include_bytes!("../../resources/VC_redist.x64.exe");
    let overrides_reg_bytes = include_bytes!("../../resources/overrides.reg");

    fs::write(tools_dir.join("wine-10.8-staging-amd64.tar.xz"), wine_bytes)?;
    fs::write(tools_dir.join("dxvk-1.10.3.tar.gz"), dxvk_bytes)?;
    fs::write(tools_dir.join("wine-mono-10.0.0-x86.msi"), wine_mono_bytes)?;
    fs::write(tools_dir.join("vc_redist.x86.exe"), vc_redist_x86_bytes)?;
    fs::write(tools_dir.join("vc_redist.x64.exe"), vc_redist_x64_bytes)?;
    fs::write(tools_dir.join("overrides.reg"), overrides_reg_bytes)?;

    Ok(())
}

pub fn extract_bundled_archives(
    tools_dir: &Path,
    runners_wine_dir: &Path,
    runtime_dxvk_dir: &Path,
) -> Result<(), CustomError> {
    let wine_dir_path = runners_wine_dir.join("wine-10.8-staging-amd64");
    let dxvk_dir_path = runtime_dxvk_dir.join("dxvk-1.10.3");

    let wine_tar_path = tools_dir.join("wine-10.8-staging-amd64.tar.xz");
    let dxvk_tar_path = tools_dir.join("dxvk-1.10.3.tar.gz");

    //create wine version directory
    fs::create_dir_all(&wine_dir_path)?;

    //create dxvk version directory
    fs::create_dir_all(&dxvk_dir_path)?;

    //extracting bundled archives
    info!("Extracting bundled tarballs");
    extract_archive(&wine_tar_path, &wine_dir_path)?;
    extract_archive(&dxvk_tar_path, &dxvk_dir_path)?;

    Ok(())
}
