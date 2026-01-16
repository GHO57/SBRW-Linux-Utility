use std::{
    fs,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use anyhow::anyhow;
use log::info;

use crate::types::error::CustomError;

pub fn check_wine_prefix(wine_prefix_path: &Path) -> Result<(), CustomError> {
    let user_reg = wine_prefix_path.join("user.reg");
    let system_reg = wine_prefix_path.join("system.reg");
    let userdef_reg = wine_prefix_path.join("userdef.reg");

    if !user_reg.exists() || !system_reg.exists() || !userdef_reg.exists() {
        return Err(CustomError::PathError(format!(
            "Missing required registry files in wine prefix {:?}",
            wine_prefix_path
        )));
    }

    info!("Validation: wine prefix found ({:?})", wine_prefix_path);

    Ok(())
}

pub fn check_game_launcher_exe(wine_prefix_path: &Path) -> Result<(), CustomError> {
    let game_launcher_path = wine_prefix_path.join("drive_c/SBRW/GameLauncher.exe");
    if !game_launcher_path.exists() {
        return Err(CustomError::PathError(format!(
            "GameLauncher.exe not found in wine prefix {:?}",
            wine_prefix_path
        )));
    }

    info!(
        "Validation: GameLauncher.exe found ({:?})",
        game_launcher_path
    );

    Ok(())
}

pub fn check_wine_bin_dir(wine_path: &Path) -> Result<(), CustomError> {
    if !wine_path.exists() {
        return Err(CustomError::PathError("Wine not found".to_string()));
    }

    info!("Validation: wine bin folder found ({:?})", wine_path);

    Ok(())
}

fn collect_dlls(path: &Path) -> Result<Vec<String>, CustomError> {
    Ok(fs::read_dir(path)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path
                .extension()?
                .to_string_lossy()
                .eq_ignore_ascii_case("dll")
            {
                Some(entry.file_name().to_string_lossy().to_string())
            } else {
                None
            }
        })
        .collect())
}

fn check_dll_symlinks(
    dlls: &[String],
    dll_dir: &Path,
    expected_path: &Path,
    arch_label: &str,
    missing: &mut Vec<String>,
) -> Result<(), CustomError> {
    for dll in dlls {
        let dll_path = dll_dir.join(dll);
        if dll_path.is_symlink() {
            let target = fs::read_link(&dll_path)?;
            if !target.starts_with(expected_path) {
                missing.push(format!(
                    "{} symlink incorrect: {:?} -> {:?}",
                    arch_label, dll_path, target
                ));
            }
        } else if !dll_path.exists() {
            missing.push(format!("{} DLL missing: {:?}", arch_label, dll_path));
        } else {
            missing.push(format!(
                "{} DLL exists but is not symlink: {:?}",
                arch_label, dll_path
            ));
        }
    }

    Ok(())
}

pub fn check_dxvk_symlinks(dxvk_path: &Path, wine_prefix_path: &Path) -> Result<(), CustomError> {
    if !dxvk_path.exists() {
        return Err(CustomError::PathError(format!(
            "provided dxvk path not found"
        )));
    }

    let dxvk_x64_path = dxvk_path.join("x64");
    let dxvk_x32_path = dxvk_path.join("x32");

    let system32_path = wine_prefix_path.join("drive_c/windows/system32");
    let syswow64_path = wine_prefix_path.join("drive_c/windows/syswow64");

    let dxvk_x64_dlls: Vec<String> = collect_dlls(&dxvk_x64_path)?;
    let dxvk_x32_dlls: Vec<String> = collect_dlls(&dxvk_x32_path)?;

    let mut missing = Vec::new();

    // Check system32 (64-bit) symlinks
    check_dll_symlinks(
        &dxvk_x64_dlls,
        &system32_path,
        &dxvk_x64_path,
        "System32",
        &mut missing,
    )?;

    // Check syswow64 (32-bit) symlinks
    check_dll_symlinks(
        &dxvk_x32_dlls,
        &syswow64_path,
        &dxvk_x32_path,
        "Syswow64",
        &mut missing,
    )?;

    if !missing.is_empty() {
        return Err(CustomError::PathError(format!(
            "DXVK symlink issues: {:?}",
            missing
        )));
    }

    info!(
        "Validation: DXVK symlinks are properly set ({:?})",
        dxvk_path
    );

    Ok(())
}

pub fn check_vc_runtimes(wine_prefix_path: &Path) -> Result<(), CustomError> {
    let system_reg_file = wine_prefix_path.join("system.reg");
    let file = fs::File::open(system_reg_file)?;
    let reader = BufReader::new(file);
    let mut section = String::new();

    let mut x64_installed = false;
    let mut x86_installed = false;

    for line in reader.lines() {
        let line = line?;

        if line.starts_with('[') {
            section = line.trim().to_string();
            continue;
        }

        if line.contains("\"Installed\"=dword:00000001") {
            if section.starts_with(r"[Software\\Microsoft\\VisualStudio\\14.0\\VC\\Runtimes\\x64]")
            {
                x64_installed = true;
            }
            if section.starts_with(
                r"[Software\\Wow6432Node\\Microsoft\\VisualStudio\\14.0\\VC\\Runtimes\\x86]",
            ) {
                x86_installed = true;
            }
        }
    }

    if !x64_installed || !x86_installed {
        return Err(CustomError::Anyhow(anyhow!(
            "VC++ runtimes are not installed correctly"
        )));
    }

    info!("Validation: Visual C++ runtimes exists");

    Ok(())
}

pub fn validate_installation(
    wine_prefix: &str,
    wine_path: &Path,
    dxvk_path: &Path,
) -> Result<(), CustomError> {
    //check wine prefix' registry files existence
    let wine_prefix_path = PathBuf::from(wine_prefix);
    check_wine_prefix(&wine_prefix_path)?;

    //check GameLauncher.exe file existence
    check_game_launcher_exe(&wine_prefix_path)?;

    //check wine bin existence
    check_wine_bin_dir(&wine_path)?;

    //check dxvk symlinks
    check_dxvk_symlinks(&dxvk_path, &wine_prefix_path)?;

    //check runtimes
    check_vc_runtimes(&wine_prefix_path)?;

    Ok(())
}
