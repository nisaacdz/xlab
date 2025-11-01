import React, { useEffect, useState, useMemo, useCallback } from "react";
import { useRecorder } from "../context/RecorderContext";
import RecordingControls from "./RecordingControls";
import RecordingOptions from "./RecordingOptions";
import { PastRecordings } from "./PastRecordings";
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
import "./RecorderMode.css";

export function RecorderMode() {
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
        if (ss && ss.state === RecordingState.SAVING) {
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
  }, [recordingState, refreshPastVideos]);

  return (
    <div className="recorder-mode">
      <div className="recorder-panel glass">
        <h2 className="card-header">Recording Studio</h2>
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
      <div className="recordings-panel glass">
        <PastRecordings
          pastVideos={pastVideos}
          removeRecording={removePastVideo}
        />
      </div>
    </div>
  );
}
