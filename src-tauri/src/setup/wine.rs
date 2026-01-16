use log::{info, warn};
use std::{
    path::{Path, PathBuf},
    thread, time,
};

use crate::{setup::check_vc_runtimes, types::error::CustomError, utils::process::run_command};

fn install_mono_runtime(
    wine_path: &Path,
    wine_prefix: &str,
    mono_msi_path: &Path,
) -> Result<(), CustomError> {
    let mono_msi_str = mono_msi_path
        .to_str()
        .ok_or_else(|| CustomError::PathError("Invalid mono path".to_string()))?;

    info!("Installing compatible wine mono to the prefix");

    run_command(
        wine_path,
        Some(&["msiexec", "/i", mono_msi_str]),
        Some(&[("WINEPREFIX", wine_prefix)]),
        None,
    )?;

    Ok(())
}

fn install_vc_runtimes(
    wine_path: &Path,
    wine_prefix: &str,
    vc_redist_x86: &Path,
    vc_redist_x64: &Path,
) -> Result<(), CustomError> {
    let vc_redist_x86_str = vc_redist_x86
        .to_str()
        .ok_or_else(|| CustomError::PathError("Invalid vc redist x86 path".to_string()))?;
    let vc_redist_x64_str = vc_redist_x64
        .to_str()
        .ok_or_else(|| CustomError::PathError("Invalid vc redist x64 path".to_string()))?;

    info!("Installing Microsoft Visual C++ runtime 2015-2022");

    //set up x86 vc redist
    run_command(
        wine_path,
        Some(&[vc_redist_x86_str, "/quiet", "/norestart"]),
        Some(&[("WINEPREFIX", wine_prefix)]),
        None,
    )?;

    //set up x64 vc redist
    run_command(
        wine_path,
        Some(&[vc_redist_x64_str, "/quiet", "/norestart"]),
        Some(&[("WINEPREFIX", wine_prefix)]),
        None,
    )?;

    let wine_prefix_path = PathBuf::from(wine_prefix);

    //waiting until installation reflects in registry files
    for attempt in 0..60 {
        thread::sleep(time::Duration::from_secs(1));

        if check_vc_runtimes(&wine_prefix_path).is_ok() {
            info!("VC++ runtimes detected after {}s", attempt + 1);
            break;
        }

        if attempt == 30 {
            warn!("Installing runtimes taking longer than expected")
        }
    }

    Ok(())
}

fn set_regedit_overrides(
    wine_path: &Path,
    wine_prefix: &str,
    overrides_reg_path: &Path,
) -> Result<(), CustomError> {
    let overrides_reg_str = overrides_reg_path
        .to_str()
        .ok_or_else(|| CustomError::PathError("Invalid registry file path".to_string()))?;

    run_command(
        wine_path,
        Some(&["regedit", "/s", overrides_reg_str]),
        Some(&[("WINEPREFIX", wine_prefix)]),
        None,
    )?;

    info!("Dll overrides set");

    Ok(())
}

pub fn install_runtimes(
    mono_msi_path: &Path,
    wine_path: &Path,
    wine_prefix: &str,
    tools_dir: &Path,
) -> Result<(), CustomError> {
    //execute wine mono msi
    install_mono_runtime(wine_path, wine_prefix, mono_msi_path)?;

    //run vcruntime with winetricks
    install_vc_runtimes(
        wine_path,
        wine_prefix,
        &tools_dir.join("vc_redist.x86.exe"),
        &tools_dir.join("vc_redist.x64.exe"),
    )?;

    //setting registry overrides
    set_regedit_overrides(wine_path, wine_prefix, &tools_dir.join("overrides.reg"))?;

    Ok(())
}

pub fn initialize_wine_environment(
    wineboot_path: &Path,
    wine_prefix: &str,
) -> Result<(), CustomError> {
    //run wineboot command to initialize isolated wine environment
    info!("Running wineboot on wine prefix");
    run_command(
        wineboot_path,
        None,
        Some(&[
            ("WINEARCH", "win64"),
            ("WINEPREFIX", wine_prefix),
            ("WINEDLLOVERRIDES", "winemenubuilder="),
        ]),
        None,
    )?;

    let wine_prefix_path = Path::new(wine_prefix);

    let user_reg = wine_prefix_path.join("user.reg");
    let system_reg = wine_prefix_path.join("system.reg");
    let userdef_reg = wine_prefix_path.join("userdef.reg");

    //waiting until registry files are created
    for attempt in 0..120 {
        thread::sleep(time::Duration::from_millis(500));
        if user_reg.exists() && system_reg.exists() && userdef_reg.exists() {
            info!("Wine registry files created after {}s.", attempt + 1);
            return Ok(());
        }

        if attempt == 60 {
            warn!("Wine initialization takes longer than expected");
        }
    }

    Err(CustomError::PathError(
        "Timed out. wine initialization failed, Try rerunning the setup".to_string(),
    ))
}
