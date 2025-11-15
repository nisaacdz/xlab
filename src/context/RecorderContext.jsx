import React, { createContext, useContext, useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

const RecorderContext = createContext();

export const useRecorder = () => useContext(RecorderContext);

export const RecorderProvider = ({ children }) => {
  const [resolution, setResolution] = useState(null);
  const [frameRate, setFrameRate] = useState(null);
  const [pointerBehavior, setPointerBehavior] = useState(null);
  const [availableResolutions, setAvailableResolutions] = useState(null);
  const [availableFrameRates, setAvailableFrameRates] = useState(null);
  const [pastVideos, setPastVideos] = useState(null);

  const updateFrameRate = async (frameRate) => {
    invoke("update_frame_rate", { frameRate }).then(() => {
      setFrameRate(frameRate);
    });
  };

  const updateResolution = async (index) => {
    invoke("update_resolution", { index }).then(() => {
      setResolution(availableResolutions[index]);
    });
  };

  const updatePointerBehavior = async (index) => {
    invoke("update_pointer", { index }).then(() => {
      setPointerBehavior(index);
    });
  };

  const refreshPastVideos = async () => {
    await invoke("past_videos").then(setPastVideos).catch(console.error);
  };

  const removePastVideo = async (index) => {
    await invoke("remove_previous_recording_by_index", { index }).then(
      refreshPastVideos,
    );
  };

  useEffect(() => {
    Promise.all([
      invoke("available_resolutions")
        .then(setAvailableResolutions)
        .catch(console.error),
      invoke("available_frame_rates")
        .then(setAvailableFrameRates)
        .catch(console.error),
      invoke("get_current_resolution")
        .then(setResolution)
        .catch(console.error),
      invoke("get_current_frame_rate")
        .then(setFrameRate)
        .catch(console.error),
      invoke("get_current_pointer")
        .then(setPointerBehavior)
        .catch(console.error),
      refreshPastVideos(),
    ]);
  }, []);

  return (
    <RecorderContext.Provider
      value={{
        resolution,
        updateResolution,
        frameRate,
        updateFrameRate,
        pointerBehavior,
        updatePointerBehavior,
        availableResolutions,
        availableFrameRates,
        pastVideos,
        refreshPastVideos,
        removePastVideo,
      }}
    >
      {children}
    </RecorderContext.Provider>
  );
};
