import { useEffect, useState } from "react";

const ProgressBar = ({ percentage }: { percentage: number }) => {
    const clamped = Math.max(0, Math.min(100, percentage));
    const [shimmerKey, setShimmerKey] = useState<number>(0);

    useEffect(() => {
        setShimmerKey((k) => k + 1);
    }, [clamped]);

    return (
        <div className="w-full bg-black/20 rounded h-2 overflow-hidden">
            <div
                className={`relative bg-black/50 dark:bg-white/70 rounded h-full overflow-hidden`}
                style={{ width: `${clamped}%` }}
            >
                {clamped < 100 && (
                    <div
                        key={shimmerKey}
                        className="absolute inset-0 bg-gradient-to-r from-transparent via-white/60 dark:via-white to-transparent animate-shimmer will-change-transform pointer-events-none"
                    />
                )}
            </div>
        </div>
    );
};

export default ProgressBar;
