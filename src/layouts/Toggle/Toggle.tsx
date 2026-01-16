const Toggle = ({
    id,
    label,
    checked,
    onChange,
    disabled = false,
}: {
    id: string;
    label: string;
    checked: boolean;
    onChange: (checked: boolean) => void;
    disabled?: boolean;
}) => {
    const handleToggle = () => {
        onChange(!checked);
    };
    return (
        <span className="flex justify-between items-center">
            <label
                htmlFor={id}
                className={`text-black dark:text-white flex min-w-fit text-[15px] ${disabled ? "opacity-60" : "opacity-100"}`}
            >
                {label}
            </label>
            <div className="w-3/4">
                <button
                    id={id}
                    role="switch"
                    onClick={handleToggle}
                    disabled={disabled}
                    className={`${checked ? "bg-dark dark:bg-white" : "bg-[#ccc] dark:bg-[#111]"} rounded-full w-14 h-fit p-1 disabled:opacity-60`}
                >
                    <div
                        className={`rounded-full transition-transform  ${checked ? "bg-light dark:bg-dark translate-x-7" : "bg-dark dark:bg-light translate-x-0"} w-5 h-5`}
                    />
                </button>
            </div>
        </span>
    );
};

export default Toggle;
