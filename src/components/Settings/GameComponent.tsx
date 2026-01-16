import TextField from "../../layouts/TextField/TextField";
import Toggle from "../../layouts/Toggle/Toggle";

interface IGame {
    game: GameSettings;
    availableCommands: IAvailableCommands;
    onChange: (game: GameSettings) => void;
}

const GameComponent = ({ game, availableCommands, onChange }: IGame) => {
    return (
        <div className="flex flex-col gap-y-3 pl-6">
            <h2 className="text-black dark:text-white font-bold -ml-6">
                Game Settings
            </h2>
            <TextField
                id="game-executable"
                label="Game Executable"
                type="text"
                value={game.launcher}
                onChange={(val) => onChange({ ...game, launcher: val })}
            />
            <TextField
                id="game-directory"
                label="Game Directory"
                type="text"
                value={game.directory}
                onChange={(val) => onChange({ ...game, directory: val })}
            />
            <TextField
                id="wine-prefix"
                label="Wine Prefix"
                type="text"
                value={game.prefix}
                onChange={(val) => onChange({ ...game, prefix: val })}
            />
            <Toggle
                id="enable-mangohud"
                label="Enable Mangohud"
                checked={availableCommands.mangohud ? game.mangohud : false}
                onChange={(mangohud) => onChange({ ...game, mangohud })}
                disabled={!availableCommands.mangohud}
            />
        </div>
    );
};

export default GameComponent;
