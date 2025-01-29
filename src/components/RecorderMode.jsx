import React, { useEffect, useState, useMemo, useCallback } from "react";
import { useRecorder } from "../context/RecorderContext";
import RecordingControls from "./RecordingControls";
import RecordingOptions from "./RecordingOptions";
import PastVideosList from "./PastVideosList";
import {
  RecordingState,
  SavingState,
  discardRecording,
  getRecordingState,
  getSavingStateAsRecordingState,
  releaseRecording,
  startRecording,
  stopRecording,
} from "../utils/api";

function RecorderMode() {
  const [recordingState, setRecordingState] = useState(null);

  const {
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
  } = useRecorder();

  const disabled = useMemo(
    () =>
      !(
        recordingState === null || recordingState.state === RecordingState.IDLE
      ),
    [recordingState],
  );

  const pollRecordingState = useCallback(async () => {
    try {
      const state = await getRecordingState();
      setRecordingState(state);
    } catch (error) {
      console.error("Error polling recording state:", error);
    }
  }, []);

  const pollSavingState = useCallback(async () => {
    try {
      const state = await getSavingStateAsRecordingState();
      console.log(state);
      setRecordingState(state);
    } catch (error) {
      console.error("Error polling saving state:", error);
    }
  }, []);

  const onStartRecording = () => {
    startRecording().then(() => pollRecordingState());
  };

  const onStopRecording = () => {
    stopRecording().then(() => pollRecordingState());
  };

  const onDiscardRecording = () => {
    discardRecording().then(() => pollRecordingState());
  };

  const onReleaseRecording = () => {
    releaseRecording().then(() => pollSavingState());
  };

  useEffect(() => {
    let timeout = null;
    if (recordingState === null) {
      getSavingStateAsRecordingState().then((ss) => {
        console.log("ss in useeffect hoook: ", ss);
        if (ss) {
          setRecordingState(ss);
        } else {
          getRecordingState().then(setRecordingState);
        }
      });
    } else if (recordingState.state === RecordingState.RECORDING) {
      timeout = setTimeout(() => {
        getRecordingState().then(setRecordingState);
      }, 200);
    } else if (recordingState.state === RecordingState.SAVING) {
      timeout = setTimeout(() => {
        if (recordingState.progress.state === SavingState.DONE) {
          refreshPastVideos().then(() =>
            setRecordingState({ state: RecordingState.IDLE }),
          );
        } else {
          getSavingStateAsRecordingState().then(setRecordingState);
        }
      }, 200);
    }

    return () => clearTimeout(timeout);
  }, [recordingState]);

  return (
    <div className="grid grid-cols-2 w-full h-full gap-6">
      <div className="glass w-full h-full p-6 flex flex-col rounded-lg gap-6">
        <h2 className="text-xl font-semibold text-center">Recorder Options</h2>
        <RecordingOptions
          resolution={resolution}
          updateResolution={updateResolution}
          frameRate={frameRate}
          updateFrameRate={updateFrameRate}
          pointerBehavior={pointerBehavior}
          updatePointerBehavior={updatePointerBehavior}
          availableResolutions={availableResolutions}
          availableFrameRates={availableFrameRates}
          disabled={disabled}
        />
        <RecordingControls
          recordingState={recordingState}
          onStartRecording={onStartRecording}
          onStopRecording={onStopRecording}
          onDiscardRecording={onDiscardRecording}
          onReleaseRecording={onReleaseRecording}
        />
      </div>
      <div className="w-full h-full relative glass p-6 rounded-lg">
        <PastVideosList
          pastVideos={pastVideos}
          removeRecording={removePastVideo}
        />
      </div>
    </div>
  );
}

export default RecorderMode;
