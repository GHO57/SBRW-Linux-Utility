const KeyValueEditor = ({ label, items, onChange }: IKeyValueEditorProps) => {
    const handleChange = (
        index: number,
        field: "key" | "value",
        newValue: string,
    ) => {
        const updated = [...items];
        updated[index][field] = newValue;
        onChange(updated);
    };

    const addItem = () => {
        onChange([...items, { key: "", value: "" }]);
    };

    const removeItem = (index: number) => {
        onChange(items.filter((_, i) => i !== index));
    };

    return (
        <div className="flex justify-between items-center">
            <h2 className="text-black dark:text-white flex min-w-fit text-[15px]">
                {label}
            </h2>
            <div className="w-3/4 flex flex-col gap-y-2">
                {items.map((item, index) => (
                    <div key={index} className="flex items-center gap-x-2">
                        <input
                            type="text"
                            value={item.key}
                            onChange={(e) =>
                                handleChange(index, "key", e.target.value)
                            }
                            placeholder="Key"
                            className="w-full text-black dark:text-white px-3 py-1 border border-gray-200 dark:border-gray-600 focus:outline-none focus:ring-2 focus:ring-black/70 dark:focus:ring-white rounded bg-white dark:bg-[#111]"
                        />
                        <input
                            type="text"
                            value={item.value}
                            onChange={(e) =>
                                handleChange(index, "value", e.target.value)
                            }
                            placeholder="Value"
                            className="w-full text-black dark:text-white px-3 py-1 border border-gray-200 dark:border-gray-600 focus:outline-none focus:ring-2 focus:ring-black/70 dark:focus:ring-white rounded bg-white dark:bg-[#111]"
                        />
                        <button
                            type="button"
                            onClick={() => removeItem(index)}
                            className="bg-red-500 text-white p-1 rounded cursor-pointer"
                        >
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                fill="none"
                                viewBox="0 0 24 24"
                                stroke-width="1.5"
                                stroke="currentColor"
                                className="size-5 text-white"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    d="M6 18 18 6M6 6l12 12"
                                />
                            </svg>
                        </button>
                    </div>
                ))}
                <button
                    type="button"
                    onClick={addItem}
                    className="w-fit bg-[#444] dark:bg-[#333] text-white px-6 py-2 rounded cursor-pointer"
                >
                    Add
                </button>
            </div>
        </div>
    );
};

export default KeyValueEditor;
