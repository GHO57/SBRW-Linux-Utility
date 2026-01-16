import { ChangeEvent } from "react";

interface ISelectProps<T = string> {
    id: string;
    label: string;
    value: T;
    onChange: (val: T) => void;
    options: { label: string; value: T }[];
}

const Select = <T extends string | number>({
    id,
    label,
    value,
    options,
    onChange,
}: ISelectProps<T>) => {
    const handleSelectionChange = (e: ChangeEvent<HTMLSelectElement>) => {
        onChange(e.target.value as T);
    };
    return (
        <span className="w-full flex justify-between items-center relative">
            <label
                htmlFor={id}
                className="text-black dark:text-white flex min-w-fit text-[15px]"
            >
                {label}
            </label>
            <select
                className="w-3/4 text-black dark:text-white px-3 py-[6px] focus:outline-none rounded bg-[#ddd] dark:bg-[#333] appearance-none"
                id={id}
                value={value}
                onChange={handleSelectionChange}
            >
                {options.map((option) => (
                    <option
                        className="rounded"
                        key={option.value}
                        value={option.value}
                    >
                        {option.label}
                    </option>
                ))}
            </select>
            <span className="pointer-events-none absolute right-3 top-1/2 -translate-y-1/2 text-white">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke-width="1.5"
                    stroke="currentColor"
                    className="size-[18px] text-black dark:text-white"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        d="m19.5 8.25-7.5 7.5-7.5-7.5"
                    />
                </svg>
            </span>
        </span>
    );
};

export default Select;
