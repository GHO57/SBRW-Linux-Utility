import { useEffect, useState } from "react";

type Theme = "light" | "dark";

export function useTheme(): [
    Theme,
    React.Dispatch<React.SetStateAction<Theme>>,
] {
    const getSystemTheme = (): Theme =>
        window.matchMedia("(prefers-color-scheme: dark)").matches
            ? "dark"
            : "light";

    const [theme, setTheme] = useState<Theme>(
        () => (localStorage.getItem("theme") as Theme) || getSystemTheme(),
    );

    useEffect(() => {
        const root = document.documentElement;
        if (theme === "dark") {
            root.classList.add("dark");
        } else {
            root.classList.remove("dark");
        }
        localStorage.setItem("theme", theme);
    }, [theme]);

    return [theme, setTheme];
}
