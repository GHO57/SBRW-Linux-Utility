import { message, open } from "@tauri-apps/plugin-dialog";
import Select from "../../layouts/Select/Select";
import Toggle from "../../layouts/Toggle/Toggle";
import { homeDir } from "@tauri-apps/api/path";
import { addWineVersion } from "../../api/tauri";
import { path } from "@tauri-apps/api";

interface IWine {
    wine: WineSettings;
    availableCommands: IAvailableCommands;
    onChange: (wine: WineSettings) => void;
    availableWineVersions: IVersions[];
}

const WineComponent = ({
    wine,
    availableCommands,
    onChange,
    availableWineVersions,
}: IWine) => {
    const handleWineVersionChange = (version: string) => {
        let path = availableWineVersions.find((v) => v.name === version)?.path;
        onChange({ ...wine, version: version, path: path as string });
    };

    const handleAddWineVersion = async () => {
        try {
            const selectedPath = await open({
                directory: false,
                multiple: false,
                defaultPath: await homeDir(),
            });

            if (typeof selectedPath !== "string") {
                return;
            }

            const fileName = await path.basename(selectedPath);

            const strippedName = fileName.replace(
                /(\.tar\.xz|\.tar\.gz)$/i,
                "",
            );

            const alreadyExists = availableWineVersions.some(
                (v) => v.name === strippedName,
            );
            if (alreadyExists) {
                await message("wine/proton version already exists", {
                    title: "Invalid",
                    kind: "error",
                });
                return;
            }

            const hasValidExtension =
                fileName.endsWith(".tar.xz") || fileName.endsWith(".tar.gz");
            const hasValidPrefix =
                strippedName.startsWith("wine-") ||
                strippedName.startsWith("Proton-") ||
                strippedName.startsWith("GE-Proton");

            if (hasValidExtension && hasValidPrefix) {
                await addWineVersion(selectedPath).then(() => {
                    message("wine/proton archive added", {
                        title: "Success",
                        kind: "info",
                    });
                });
            } else {
                await message("Invalid wine/proton archive", {
                    title: "Invalid",
                    kind: "error",
                });
            }
        } catch (error) {
            await message(String(error), {
                title: "Something Went Wrong",
                kind: "error",
            });
        }
    };

    return (
        <>
            <div className="flex flex-col gap-y-3 pl-6">
                <h2 className="text-black dark:text-white font-bold -ml-6">
                    Wine Settings
                </h2>
                <div className="flex items-center gap-x-4">
                    <Select
                        id="wine-version"
                        label="Wine version"
                        value={wine.version}
                        onChange={(version) => handleWineVersionChange(version)}
                        options={availableWineVersions.map((v) => ({
                            label: v.name,
                            value: v.name,
                        }))}
                    />
                    <button
                        onClick={handleAddWineVersion}
                        className="bg-white dark:bg-[#444] w-fit h-full p-2 rounded cursor-pointer"
                    >
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke-width="1.5"
                            stroke="currentColor"
                            className="size-5 text-black dark:text-white"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                d="M4.5 10.5 12 3m0 0 7.5 7.5M12 3v18"
                            />
                        </svg>
                    </button>
                </div>
                <Toggle
                    id="enable-esync"
                    label="Enable Esync"
                    checked={wine.esync}
                    onChange={(esync) => onChange({ ...wine, esync })}
                />
                <Toggle
                    id="enable-fsync"
                    label="Enable Fsync"
                    checked={wine.fsync}
                    onChange={(fsync) => onChange({ ...wine, fsync })}
                />
                <Toggle
                    id="enable-feral-gamemode"
                    label="Enable Feral GameMode"
                    checked={
                        availableCommands.gamemoderun
                            ? wine.feral_gamemode
                            : false
                    }
                    onChange={(feral_gamemode) =>
                        onChange({ ...wine, feral_gamemode })
                    }
                    disabled={!availableCommands.gamemoderun}
                />
            </div>
        </>
    );
};

export default WineComponent;
