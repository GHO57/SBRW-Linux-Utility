import { invoke } from "@tauri-apps/api/core";

//start setting up
export const startSettingUp = async (options: any): Promise<string> => {
    return await invoke<string>("setup_environment", {
        options: {
            wine_prefix: options.winePrefix,
            // desktop_shortcut_needed: options.desktopShorcutChecked,
            // menu_shortcut_needed: options.menuShortcutChecked,
        },
    });
};

//check config.toml whether prompt user to install or launch game
export const checkConfig = async () => {
    return await invoke<boolean>("check_config");
};

//launch game
export const launchGame = async () => {
    return await invoke<void>("launch_game");
};

//stop game
export const stopGame = async () => {
    return await invoke<void>("stop_game");
};

//load config file to populate settings
export const populateSettings = async () => {
    return await invoke<IPopulateSettings>("populate_settings");
};

//get available system commnds
export const getAvailableCommands = async () => {
    return await invoke<IAvailableCommands>("get_command_availability");
};

//save settings
export const saveSettings = async (config: IPopulateSettings) => {
    return await invoke<boolean>("save_settings", {
        config,
    });
};

//add new wine version
export const addWineVersion = async (selectedPath: string) => {
    return await invoke<void>("add_wine_version", {
        selectedPath,
    });
};
