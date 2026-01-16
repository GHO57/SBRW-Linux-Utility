/*
interfaces for hooks
*/
interface IListener<T> {
    onStateUpdate: (state: T) => void;
    eventName: string;
}

/*
interfaces for components & layouts
*/
interface IKeyValue {
    key: string;
    value: string;
}

interface IKeyValueEditorProps {
    label: string;
    items: IKeyValue[];
    onChange: (items: IKeyValue[]) => void;
}

/*
interfaces for api
*/
interface IVersions {
    name: string;
    path: string;
}

type WineSettings = {
    version: string;
    path: string;
    esync: boolean;
    fsync: boolean;
    feral_gamemode: boolean;
};

type DXVKSettings = {
    version: string;
    path: string;
    enabled: boolean;
};

type GameSettings = {
    launcher: string;
    directory: string;
    prefix: string;
    mangohud: boolean;
    environment_variables: IKeyValue[];
    dll_overrides: IKeyValue[];
};

type SettingsState = {
    game: GameSettings;
    wine: WineSettings;
    dxvk: DXVKSettings;
    availableWineVersions?: IVersions[];
    availableDXVKVersions?: IVersions[];
};

interface IPopulateSettings {
    wine: WineSettings;
    dxvk: DXVKSettings;
    game: GameSettings;
    available_wine_versions?: IVersions[];
    available_dxvk_versions?: IVersions[];
}

interface IAvailableCommands {
    mangohud: boolean;
    gamemoderun: boolean;
}
