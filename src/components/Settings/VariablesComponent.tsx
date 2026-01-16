import KeyValueEditor from "../../layouts/KeyValueEditor/KeyValueEditor";

interface IVariables {
    game: GameSettings;
    availableCommands: IAvailableCommands;
    onChange: (game: GameSettings) => void;
}

const VariablesComponent = ({
    game,
    availableCommands,
    onChange,
}: IVariables) => {
    const handleEnvChange = (env_arr: IKeyValue[]) => {
        onChange({ ...game, environment_variables: env_arr });
    };

    const handleDllChange = (dll_arr: IKeyValue[]) => {
        onChange({ ...game, dll_overrides: dll_arr });
    };
    return (
        <div className="flex flex-col gap-y-3 pl-6">
            <h2 className="text-black dark:text-white font-bold -ml-6">
                Variables Settings
            </h2>
            <div className="w-full flex flex-col gap-y-12">
                <KeyValueEditor
                    label="Environment Variables"
                    items={game.environment_variables}
                    onChange={(env_arr) => handleEnvChange(env_arr)}
                />
                <KeyValueEditor
                    label="DLL Overrides"
                    items={game.dll_overrides}
                    onChange={(dll_arr) => handleDllChange(dll_arr)}
                />
            </div>
        </div>
    );
};

export default VariablesComponent;
