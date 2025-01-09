import React, { createContext, useContext, useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

const RecorderContext = createContext();

export const useRecorder = () => useContext(RecorderContext);

const DEFAULT_RESOLUTION = 720;
const DEFAULT_FRAME_RATE = 30;

export const RecorderProvider = ({ children }) => {
    const [resolution, setResolution] = useState(() => {
        const old_width = window.screen.width;
        const old_height = window.screen.height;
        const new_height = DEFAULT_RESOLUTION;
        const new_width = Math.floor((old_width / old_height) * new_height);
        return [new_width, new_height];
    });
    const [frameRate, setFrameRate] = useState(DEFAULT_FRAME_RATE);
    const [pointerBehavior, setPointerBehavior] = useState("system");
    const [availableResolutions, setAvailableResolutions] = useState(null);
    const [availableFrameRates, setAvailableFrameRates] = useState(null);
    const [pastVideos, setPastVideos] = useState(null);

    const refreshPastVideos = async () => {
        await invoke("past_videos")
            .then(setPastVideos)
            .catch(console.error);
    };

    useEffect(() => {
        Promise.all([
            invoke("available_resolutions")
                .then(setAvailableResolutions)
                .catch(console.error),
            invoke("available_frame_rates")
                .then(setAvailableFrameRates)
                .catch(console.error),
            refreshPastVideos(),
        ]);
    }, []);

    return (
        <RecorderContext.Provider
            value={{
                resolution,
                setResolution,
                frameRate,
                setFrameRate,
                pointerBehavior,
                setPointerBehavior,
                availableResolutions,
                availableFrameRates,
                pastVideos,
                refreshPastVideos,
            }}
        >
            {children}
        </RecorderContext.Provider>
    );
};
