use log::info;
use std::fs;
use tauri::path::BaseDirectory;
use tauri::Window;
use tauri::{AppHandle, Manager};

use crate::setup::dxvk::setup_dxvk;
use crate::setup::finalize::finalize_setup;
use crate::setup::resource::{
    download_game_launcher, extract_bundled_archives, write_bundled_resources,
};
use crate::setup::validate::validate_installation;
use crate::setup::wine::{initialize_wine_environment, install_runtimes};
use crate::types::error::CustomError;
use crate::types::setup::SetupOptions;
use crate::types::wizard::WizardStep;
use crate::utils::emitter::report_step;
use crate::utils::fs::cleanup_dir;

#[tauri::command]
pub fn setup_environment(
    options: SetupOptions,
    window: Window,
    app: AppHandle,
) -> Result<(), CustomError> {
    std::thread::spawn(move || -> Result<(), CustomError> {
        info!("Setup started");

        //Get the app dir which is ~/.local/share/sbrw-utility
        let app_dir = app.path().resolve("sbrw-utility", BaseDirectory::Data)?;

        report_step(&window, WizardStep::Cleanup)?;
        cleanup_dir(&app_dir, "app")?;

        report_step(&window, WizardStep::SetupFolders)?;

        let wine_version = "wine-10.8-staging-amd64";
        let dxvk_version = "dxvk-1.10.3";
        let mono_version = "wine-mono-10.0.0-x86.msi";

        //Creating necessary directories
        let runners_dir = app_dir.join("runners");
        let runtime_dir = app_dir.join("runtime");
        let tools_dir = app_dir.join("tools");

        let wine_prefix = options.wine_prefix();
        let runners_wine_dir = runners_dir.join("wine");
        let runners_proton_dir = runners_dir.join("proton");
        let runtime_dxvk_dir = runtime_dir.join("dxvk");

        let wine_path = runners_wine_dir.join(wine_version);
        let dxvk_path = runtime_dxvk_dir.join(dxvk_version);
        let mono_msi_path = tools_dir.join(mono_version);

        fs::create_dir_all(&runners_wine_dir)?;
        fs::create_dir_all(&runners_proton_dir)?;
        fs::create_dir_all(&runtime_dxvk_dir)?;
        fs::create_dir_all(&tools_dir)?;

        report_step(&window, WizardStep::DownloadLauncher)?;
        let launcher_path = download_game_launcher(&tools_dir)?;

        report_step(&window, WizardStep::WriteResources)?;
        write_bundled_resources(&tools_dir)?;

        report_step(&window, WizardStep::ExtractResources)?;
        extract_bundled_archives(&tools_dir, &runners_wine_dir, &runtime_dxvk_dir)?;

        report_step(&window, WizardStep::InitializeWine)?;
        initialize_wine_environment(&wine_path.join("bin/wineboot"), &wine_prefix)?;

        report_step(&window, WizardStep::InstallRuntimes)?;
        install_runtimes(
            &mono_msi_path,
            &wine_path.join("bin/wine"),
            &wine_prefix,
            &tools_dir,
        )?;

        report_step(&window, WizardStep::SetupDXVK)?;
        setup_dxvk(&wine_prefix, &dxvk_path)?;

        report_step(&window, WizardStep::FinalizeSetup)?;
        finalize_setup(
            &wine_prefix,
            &launcher_path,
            &app_dir,
            &wine_version,
            &wine_path.join("bin"),
            &dxvk_version,
            &dxvk_path,
        )?;

        report_step(&window, WizardStep::ValidateInstallation)?;
        validate_installation(&wine_prefix, &wine_path.join("bin"), &dxvk_path).map_err(
            |error| {
                report_step(&window, WizardStep::Failed).ok();
                info!("Installation validation failed {0}", error);
                error
            },
        )?;
        info!("Validation successful, everything's in place");

        report_step(&window, WizardStep::Complete)?;
        info!("Setup completed successfully");

        Ok(())
    });

    Ok(())
}
