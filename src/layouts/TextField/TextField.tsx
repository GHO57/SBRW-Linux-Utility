import { ChangeEvent } from "react";

const TextField = ({
    id,
    label,
    type,
    value,
    onChange,
}: {
    id: string;
    label: string;
    type: string;
    value: string;
    onChange: (val: string) => void;
}) => {
    const handleTextFieldChange = (e: ChangeEvent<HTMLInputElement>) => {
        onChange(e.target.value);
    };
    return (
        <span className="flex justify-between items-center">
            <label
                htmlFor={id}
                className="text-black dark:text-white flex min-w-fit text-[15px]"
            >
                {label}
            </label>
            <input
                id={id}
                className="w-3/4 text-black dark:text-white px-3 py-1 border border-gray-300 dark:border-gray-700 focus:outline-none focus:ring-2 focus:ring-black/70 dark:focus:ring-white rounded bg-white dark:bg-[#111]"
                type={type}
                value={value}
                onChange={handleTextFieldChange}
            />
        </span>
    );
};

export default TextField;
