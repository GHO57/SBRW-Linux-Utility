import Select from "../../layouts/Select/Select";
import Toggle from "../../layouts/Toggle/Toggle";

interface IDXVK {
    dxvk: DXVKSettings;
    availableCommands: IAvailableCommands;
    onChange: (dxvk: DXVKSettings) => void;
    availableDXVKVersions: IVersions[];
}

const DXVKComponent = ({
    dxvk,
    availableCommands,
    onChange,
    availableDXVKVersions,
}: IDXVK) => {
    const handleDXVKVersionChange = (version: string) => {
        let path = availableDXVKVersions.find((v) => v.name === version)?.path;
        onChange({ ...dxvk, version: version, path: path as string });
    };
    return (
        <div className="flex flex-col gap-y-3 pl-6">
            <h2 className="text-black dark:text-white font-bold -ml-6">
                DXVK Settings
            </h2>
            <Toggle
                id="enable-dxvk"
                label="Enable DXVK"
                checked={dxvk.enabled}
                onChange={(enabled) => onChange({ ...dxvk, enabled })}
            />
            <Select
                id="dxvk-version"
                label="DXVK version"
                value={dxvk.version}
                onChange={(version) => handleDXVKVersionChange(version)}
                options={availableDXVKVersions.map((v) => ({
                    label: v.name,
                    value: v.name,
                }))}
            />
        </div>
    );
};

export default DXVKComponent;
