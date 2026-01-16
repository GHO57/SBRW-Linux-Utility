import { listen } from "@tauri-apps/api/event";
import { message } from "@tauri-apps/plugin-dialog";
import { useEffect } from "react";
import { useNavigate } from "react-router-dom";

const messages: Record<string, string> = {
    Cleanup: "Cleaning up...",
    SetupFolders: "Setting up folders...",
    DownloadLauncher: "Downloading game launcher...",
    WriteResources: "Copying bundled resources...",
    ExtractResources: "Extracting bundled archives...",
    InitializeWine: "Initializing Wine environment...",
    InstallRuntimes: "Installing runtimes...",
    SetupDXVK: "Setting up DXVK...",
    FinalizeSetup: "Finalizing setup...",
    ValidateInstallation: "Validating installation...",
    Complete: "Setup completed successfully!",
    Failed: "Setup failed, rerun the setup",
};

const stepProgress: Record<string, number> = {
    Cleanup: 8,
    SetupFolders: 10,
    DownloadLauncher: 15,
    WriteResources: 25,
    ExtractResources: 40,
    InitializeWine: 65,
    InstallRuntimes: 75,
    SetupDXVK: 85,
    FinalizeSetup: 90,
    ValidateInstallation: 95,
    Complete: 100,
    Failed: 100,
};

export const WizardListener = ({
    onStepUpdate,
    onProgressUpdate,
}: {
    onStepUpdate: (step: string) => void;
    onProgressUpdate: (progress: number) => void;
}) => {
    const navigate = useNavigate();
    useEffect(() => {
        let unlistenStep: (() => void) | null = null;

        const setupListeners = async () => {
            unlistenStep = await listen<string>(
                "wizard-step",
                async (event) => {
                    let step = event.payload;
                    onStepUpdate(messages[step]);
                    onProgressUpdate(stepProgress[step]);

                    if (step === "Complete") {
                        await message("Setup completed successfully").then(
                            () => {
                                navigate("/");
                            },
                        );
                    } else if (step === "Failed") {
                        await message("Setup failed, rerun it", {
                            // title: "Failed",
                            kind: "error",
                        }).then(() => {
                            navigate("/");
                        });
                    }
                },
            );
        };

        setupListeners();

        return () => {
            if (unlistenStep) unlistenStep();
        };
    }, []);

    return null;
};
