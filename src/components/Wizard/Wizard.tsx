import { useEffect, useRef, useState } from "react";
import { Link } from "react-router-dom";
import { homeDir } from "@tauri-apps/api/path";
import { open } from "@tauri-apps/plugin-dialog";
import ProgressBar from "../../layouts/ProgressBar/ProgressBar";
import { startSettingUp } from "../../api/tauri";
import { WizardListener } from "./WizardListener";

const SetupOptions = ({ onNext }: { onNext: (options: any) => void }) => {
    const [homeDirPath, setHomeDirPath] = useState<string>("");
    const [winePrefix, setWinePrefix] = useState<string>("");
    const [desktopShortcutChecked, setDesktopShortcutChecked] =
        useState<boolean>(false);
    const [menuShortcutChecked, setMenuShortcutChecked] =
        useState<boolean>(false);

    const handleGetWinePrefix = async () => {
        const newWinePrefix: string | null = await open({
            directory: true,
            multiple: false,
            defaultPath: homeDirPath,
        });

        if (typeof newWinePrefix === "string") {
            setWinePrefix(newWinePrefix);
        }
    };

    const handleSubmit = () => {
        onNext({ winePrefix, desktopShortcutChecked, menuShortcutChecked });
    };

    useEffect(() => {
        const getDefaultWinePrefix = async () => {
            const path: string = await homeDir();
            const defaultPrefix: string = `${path}/Games/need-for-speed-world`;
            setHomeDirPath(path);
            setWinePrefix(defaultPrefix);
        };

        getDefaultWinePrefix();

        return () => {
            setHomeDirPath("");
            setWinePrefix("");
        };
    }, []);

    return (
        <>
            <div className="flex flex-col items-center justify-between h-full w-full">
                <div className="flex flex-col justify-center items-center gap-y-4 w-full">
                    <p className="text-black dark:text-white">
                        Select game installation folder
                    </p>
                    <div className="w-full flex flex-col gap-y-1">
                        <div className="w-full flex items-center justify-between gap-x-3">
                            <input
                                className="w-full text-white px-3 py-1 border border-gray-600 focus:outline-none focus:ring-2 focus:ring-white rounded bg-[#222] dark:bg-[#111]"
                                type="text"
                                value={winePrefix}
                                onChange={(e) => setWinePrefix(e.target.value)}
                                required
                            />
                            <button
                                onClick={handleGetWinePrefix}
                                className="max-w-9 max-h-9 w-full h-full  bg-white dark:bg-[#444]  rounded p-2 flex items-center justify-center hover:bg-[#555] cursor-pointer"
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
                                        d="M6.75 12a.75.75 0 1 1-1.5 0 .75.75 0 0 1 1.5 0ZM12.75 12a.75.75 0 1 1-1.5 0 .75.75 0 0 1 1.5 0ZM18.75 12a.75.75 0 1 1-1.5 0 .75.75 0 0 1 1.5 0Z"
                                    />
                                </svg>
                            </button>
                        </div>
                        <p className="text-black dark:text-white text-sm w-full">
                            <b>Note</b>: Always select an empty dedicated folder
                            for the game to get rid of installation issues.
                        </p>
                    </div>
                    <div className="flex flex-col justify-start w-full gap-y-2">
                        <label className="w-full flex items-center gap-x-2 text-black dark:text-white cursor-pointer">
                            <input
                                type="checkbox"
                                checked={desktopShortcutChecked}
                                onChange={() =>
                                    setDesktopShortcutChecked((prev) => !prev)
                                }
                            />
                            <p>Create desktop shortcut</p>
                        </label>
                        <label className="w-full flex items-center gap-x-2 text-black dark:text-white cursor-pointer">
                            <input
                                type="checkbox"
                                checked={menuShortcutChecked}
                                onChange={() =>
                                    setMenuShortcutChecked((prev) => !prev)
                                }
                            />
                            <p>Create application menu shortcut</p>
                        </label>
                    </div>
                    {/*<div className="w-full flex">
                        <p className="text-primary">Warning</p>: Couldn't find
                        the follow system commands -
                        {missingCommands.map((cmd, index) => (
                            <p className="mx-1">
                                {cmd}
                                {index == missingCommands.length - 1 ? "" : ","}
                            </p>
                        ))}
                    </div>*/}
                </div>
                <div className="w-full flex items-center justify-between">
                    <Link
                        className="button px-5 py-2 rounded-md font-medium shadow-md dark:shadow-none"
                        to="/"
                    >
                        Cancel
                    </Link>
                    <div className="flex items-center gap-x-4">
                        {/*<button className="bg-[#555] px-4 py-2 rounded-md text-white cursor-pointer disabled:opacity-50 disabled:cursor-default">
                            Back
                        </button>*/}
                        <button
                            onClick={handleSubmit}
                            // disabled={missingCommands.length > 0}
                            className="bg-white hover:bg-gray-100 transition-all duration-100 shadow-md dark:shadow-none px-5 py-2 rounded-md font-medium cursor-pointer disabled:pointer-events-none disabled:opacity-50"
                        >
                            Install
                        </button>
                    </div>
                </div>
            </div>
        </>
    );
};

const Installation = ({ options }: { options: any }) => {
    const [step, setStep] = useState<string>("Starting setup");
    const [progressPercentage, setProgressPercentage] = useState<number>(5);

    const setupStarted = useRef(false);

    useEffect(() => {
        if (!setupStarted.current) {
            setupStarted.current = true;
            const start = async () => {
                try {
                    await startSettingUp(options);
                } catch (error) {
                    console.error("Setup failed:", error);
                }
            };
            start();
        }
    }, []);

    return (
        <div className="w-full h-full justify-center items-center">
            <div className="flex flex-col gap-y-8">
                <h2 className="text-black dark:text-white font-medium text-center text-xl">
                    please wait for a couple of minutes
                </h2>
                <div>
                    <p className="text-black dark:text-white">{step}</p>
                    <ProgressBar percentage={progressPercentage} />
                </div>
            </div>
            <div></div>
            <WizardListener
                onStepUpdate={(msg) => setStep(msg)}
                onProgressUpdate={(val) => setProgressPercentage(val)}
            />
        </div>
    );
};

const Wizard = () => {
    const [step, setStep] = useState<1 | 2>(1);
    const [options, setOptions] = useState<any>(null);
    return (
        <div className="w-full h-full px-20 py-10">
            {step === 1 && (
                <SetupOptions
                    onNext={(opts) => {
                        setOptions(opts);
                        setStep(2);
                    }}
                />
            )}
            {step === 2 && <Installation options={options} />}
        </div>
    );
};

export default Wizard;
