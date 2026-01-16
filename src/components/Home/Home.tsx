import { useEffect, useState } from "react";
import { Link } from "react-router-dom";
import { checkConfig, launchGame, stopGame } from "../../api/tauri";
import { Listener } from "./Listener";
import TopBar from "../TopBar/TopBar";

type GameState = "GameLaunching" | "GameRunning" | "GameStopped";

const gameStateMessages: Record<GameState, string> = {
    GameLaunching: "Launching...",
    GameRunning: "Stop Game",
    GameStopped: "Launch Game",
};

const Home = () => {
    type ConfigState = "loading" | "ready" | "broken";
    let [launchableState, setLaunchableState] =
        useState<ConfigState>("loading");
    let [gameState, setGameState] = useState<GameState>("GameStopped");

    const handleLaunchGame = async () => {
        try {
            if (gameState === "GameStopped") {
                await launchGame();
            } else if (gameState === "GameRunning") {
                await stopGame();
            }
        } catch (error) {
            console.log("Couldn't launch the game");
        }
    };

    useEffect(() => {
        const startCheck = async () => {
            try {
                await checkConfig().then((response) => {
                    setLaunchableState(response ? "ready" : "broken");
                });
            } catch (error) {
                console.error("Couldn't check config:", error);
            }
        };

        startCheck();
    }, []);

    return (
        <>
            <div className="flex flex-col items-center w-full h-full">
                <TopBar />
                <div className="w-full h-[82%] flex flex-col items-center justify-center gap-y-12">
                    <div className="text-center max-w-120">
                        <p className="text-black dark:text-white/75 font-light">
                            Welcome To
                        </p>
                        <p className="text-black dark:text-white text-3xl font-bold">
                            SBRW Utility for Linux
                        </p>
                        <p className="text-black dark:text-white/50 mt-2">
                            SoapBox Race World(SBRW) Utility provides automated
                            setup for the game on linux and provides advanced
                            options if you ever want to tweak things manually.
                        </p>
                    </div>
                    <div className="flex justify-between gap-x-4">
                        {launchableState === "loading" ? (
                            <button
                                className="px-8 py-4 rounded-md font-medium disabled:bg-gray-700 disabled:opacity-50 disabled:pointer-events-none flex justify-center items-center"
                                disabled={true}
                            >
                                <div className="text-white flex justify-center items-center gap-x-2">
                                    <p>Validating</p>
                                    <div className="border-2 border-white border-r-transparent w-5 h-5 rounded-full animate-[spin_0.5s_linear_infinite]" />
                                </div>
                            </button>
                        ) : launchableState === "ready" ? (
                            <button
                                onClick={handleLaunchGame}
                                className={`${gameState === "GameRunning" ? "bg-primary hover:bg-primary-hover" : "bg-green-600 hover:bg-green-700"} text-white duration-100 transition-all px-8 py-4 rounded-md font-medium cursor-pointer disabled:bg-gray-700 disabled:opacity-50 disabled:pointer-events-none`}
                                disabled={gameState === "GameLaunching"}
                            >
                                {gameStateMessages[gameState]}
                            </button>
                        ) : (
                            <Link
                                to="/setup-wizard"
                                className="button px-8 py-4 rounded-md font-medium"
                            >
                                Start Setup
                            </Link>
                        )}
                    </div>
                </div>
            </div>
            <Listener
                onStateUpdate={(msg: GameState) => setGameState(msg)}
                eventName="game-state"
            />
        </>
    );
};

export default Home;
