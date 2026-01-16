use log::info;
use std::fs;
use std::os::unix::fs as unix_fs;
use std::path::Path;
use std::process::Command;

use crate::types::error::CustomError;

pub fn cleanup_dir(dir_path: &Path, dir_name: &str) -> Result<(), CustomError> {
    //Clean up if there exists sbrw-utility directory
    if dir_path.exists() {
        info!("Cleaning up existing {0}", dir_name);
        let _ = fs::remove_dir_all(dir_path)?;
    }

    Ok(())
}

pub fn cleanup_file(file_path: &Path) -> Result<(), CustomError> {
    if file_path.exists() {
        info!(
            "Cleaning up file in path: {}",
            file_path.display().to_string()
        );
        let _ = fs::remove_file(file_path)?;
    }

    Ok(())
}

pub fn extract_tarball(file: &Path, dest: &Path) -> Result<(), CustomError> {
    //Function to extract tarballs

    let file_str = file.to_str().expect("Invalid file path");
    let dest_str = dest.to_str().expect("Invalid destination path");

    Command::new("tar")
        .args(&["-xf", file_str, "-C", dest_str, "--strip-components=1"])
        .status()?;

    Ok(())
}

pub fn extract_zip(file: &Path, dest: &Path) -> Result<(), CustomError> {
    //Function to extract zip files

    let file_str = file.to_str().expect("Invalid file path");
    let dest_str = dest.to_str().expect("Invalid destination path");

    Command::new("unzip")
        .args(&[file_str, "-d", dest_str])
        .status()?;

    Ok(())
}

pub fn extract_archive(file: &Path, dest: &Path) -> Result<(), CustomError> {
    // Wrapper function for extract_tar
    // and extract_zip functions

    let ext = file
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        "zip" => extract_zip(file, dest),
        "gz" | "xz" | "bz2" | "tar" => extract_tarball(file, dest),
        other => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Unsupported archive format: {}", other),
        ))?,
    }
}

pub fn create_symlinks_by_filename(
    source_dir: &Path,
    dest_dir: &Path,
    forceable: bool,
) -> Result<(), CustomError> {
    for entry in fs::read_dir(source_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let file_name = match path.file_name() {
                Some(name) => name,
                None => continue,
            };

            let source_file_path = source_dir.join(&file_name);
            let dest_file_path = dest_dir.join(&file_name);

            if forceable && dest_file_path.exists() {
                fs::remove_file(&dest_file_path)?;
            }

            unix_fs::symlink(&source_file_path, &dest_file_path)?;
        }
    }

    Ok(())
}
