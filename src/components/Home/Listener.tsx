import { listen } from "@tauri-apps/api/event";
import { useEffect } from "react";

export function Listener<T extends string>({
    onStateUpdate,
    eventName,
}: IListener<T>) {
    useEffect(() => {
        let unlistenState: (() => void) | null = null;

        const setupListeners = async () => {
            unlistenState = await listen<T>(eventName, async (event) => {
                let state = event.payload;
                onStateUpdate(state);
            });
        };

        setupListeners();

        return () => {
            if (unlistenState) unlistenState();
        };
    }, []);

    return null;
}
