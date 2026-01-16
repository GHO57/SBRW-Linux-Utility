use serde::Serialize;

#[derive(Serialize, Clone)]
pub enum WizardStep {
    Cleanup,
    SetupFolders,
    DownloadLauncher,
    WriteResources,
    ExtractResources,
    InitializeWine,
    InstallRuntimes,
    SetupDXVK,
    FinalizeSetup,
    ValidateInstallation,
    Complete,
    Failed,
}
