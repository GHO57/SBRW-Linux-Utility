import { useEffect, useState } from "react";
import {
    getAvailableCommands,
    populateSettings,
    saveSettings,
} from "../../api/tauri";
import { Link, useNavigate } from "react-router-dom";
import GameComponent from "./GameComponent";
import DXVKComponent from "./DXVKComponent";
import WineComponent from "./WineComponent";
import VariablesComponent from "./VariablesComponent";

const Settings = () => {
    const navigate = useNavigate();

    const [loading, setLoading] = useState<boolean>(true);

    const [wineState, setWineState] = useState<WineSettings>({
        version: "",
        path: "",
        esync: false,
        fsync: false,
        feral_gamemode: false,
    });

    const [dxvkState, setDXVKState] = useState<DXVKSettings>({
        version: "",
        path: "",
        enabled: false,
    });

    const [gameState, setGameState] = useState<GameSettings>({
        launcher: "",
        directory: "",
        prefix: "",
        mangohud: false,
        environment_variables: [],
        dll_overrides: [],
    });

    const [availableWineVersions, setAvailableWineVersions] = useState<
        IVersions[]
    >([]);
    const [availableDXVKVersions, setAvailableDXVKVersions] = useState<
        IVersions[]
    >([]);

    const [availableCommands, setAvailableCommands] =
        useState<IAvailableCommands>({
            mangohud: false,
            gamemoderun: false,
        });

    const handleSettingsSave = async () => {
        const config = {
            wine: {
                version: wineState.version,
                path: wineState.path,
                esync: wineState.esync,
                fsync: wineState.fsync,
                feral_gamemode: wineState.feral_gamemode,
            },
            dxvk: {
                version: dxvkState.version,
                path: dxvkState.path,
                enabled: dxvkState.enabled,
            },
            game: {
                prefix: gameState.prefix,
                directory: gameState.directory,
                launcher: gameState.launcher,
                mangohud: gameState.mangohud,
                environment_variables: gameState.environment_variables,
                dll_overrides: gameState.dll_overrides,
            },
        };
        await saveSettings(config).then((response) => {
            if (response) {
                navigate("/");
            }
        });
    };

    useEffect(() => {
        const populate = async () => {
            try {
                const [config, availability] = await Promise.all([
                    populateSettings(),
                    getAvailableCommands(),
                ]);
                setWineState(config.wine);
                setDXVKState(config.dxvk);
                setGameState(config.game);
                setAvailableWineVersions(config.available_wine_versions || []);
                setAvailableDXVKVersions(config.available_dxvk_versions || []);
                setAvailableCommands(availability);
            } finally {
                setLoading(false);
            }
        };

        populate();
    }, []);

    return (
        <>
            {!loading && (
                <div className="w-full h-full bg-light dark:bg-dark flex flex-col justify-center items-center">
                    <div className="w-full h-full px-12 py-8 flex flex-col gap-y-10 overflow-y-auto">
                        <GameComponent
                            game={gameState}
                            availableCommands={availableCommands}
                            onChange={setGameState}
                        />
                        <WineComponent
                            wine={wineState}
                            availableCommands={availableCommands}
                            onChange={setWineState}
                            availableWineVersions={availableWineVersions}
                        />
                        <DXVKComponent
                            dxvk={dxvkState}
                            availableCommands={availableCommands}
                            onChange={setDXVKState}
                            availableDXVKVersions={availableDXVKVersions}
                        />
                        <VariablesComponent
                            game={gameState}
                            availableCommands={availableCommands}
                            onChange={setGameState}
                        />
                    </div>
                    <div className="w-full px-12 py-4 flex items-center justify-between">
                        <Link
                            className="button px-5 py-2 rounded-md font-medium shadow-md dark:shadow-none"
                            to="/"
                        >
                            Cancel
                        </Link>
                        <button
                            className="bg-white hover:bg-gray-100 transition-all duration-100 shadow-md dark:shadow-none px-5 py-2 rounded-md font-medium cursor-pointer disabled:pointer-events-none disabled:opacity-50"
                            onClick={handleSettingsSave}
                        >
                            Save
                        </button>
                    </div>
                </div>
            )}
        </>
    );
};

export default Settings;
