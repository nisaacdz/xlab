import React, { useEffect, useState } from "react";
import { useRecorder } from "../context/RecorderContext";
import { invoke } from "@tauri-apps/api/core";
import { X } from "lucide-react";
import renderSolidPointer from "./SolidPointers";
import {
  Select,
  SelectTrigger,
  SelectValue,
  SelectContent,
  SelectItem,
} from "@/components/ui/select";

const RecordingState = Object.freeze({
  IDLE: "Idle",
  RECORDING: "Recording",
  DONE: "Done",
  SAVING: "Saving",
  DISCARDING: "Discarding",
});

const SavingState = Object.freeze({
  INITIALIZING: "Initializing",
  SAVING: "Saving",
  FINALIZING: "Finalizing",
  DONE: "Done",
});

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
  } = useRecorder();

  useEffect(() => {
    if (recordingState === null) {
      getRecordingState().then(setRecordingState);
    } else if (recordingState.state === RecordingState.RECORDING) {
      setTimeout(() => {
        getRecordingState().then(setRecordingState);
      }, 200);
    } else if (recordingState.state === RecordingState.SAVING) {
      setTimeout(() => {
        if (recordingState.progress.state === SavingState.DONE) {
          setRecordingState({ state: RecordingState.IDLE });
          refreshPastVideos();
        } else {
          getSavingStateAsRecordingState().then(setRecordingState);
        }
      }, 200);
    }
  }, [recordingState]);

  const disabled = !(
    recordingState === null || recordingState.state === RecordingState.IDLE
  );

  const onStartRecording = () => {
    invoke("start_recording").then(() => {
      getRecordingState()
        .then(setRecordingState)
        .catch((e) => console.error(e));
    });
  };

  const onStopRecording = () => {
    invoke("stop_recording").then(() => {
      getRecordingState()
        .then(setRecordingState)
        .catch((e) => console.error(e));
    });
  };

  const handleDiscardVideo = () => {
    setRecordingState({ state: RecordingState.DISCARDING });
    invoke("discard_recording").then(() => {
      setRecordingState(null);
    });
  };

  const handleReleaseVideo = () => {
    invoke("save_recording").then(() => {
      getSavingStateAsRecordingState().then(setRecordingState);
    });
  };

  return (
    <div className="grid grid-cols-2 w-full h-full gap-6">
      <div className="glass w-full h-full p-6 rounded-lg grid grid-cols-1 gap-6">
        <h2 className="text-xl font-semibold text-center">Recorder Options</h2>

        {/* Resolution Selection */}
        <div className="flex flex-col sm:flex-row sm:items-center gap-4">
          <label className="text-lg font-medium">Resolution:</label>
          <select
            className="glass p-2 rounded-md w-full text-slate-800"
            value={resolution[1]}
            onChange={(e) => updateResolution(e.target.selectedIndex)}
            disabled={disabled}
          >
            {availableResolutions &&
              availableResolutions.map((res, index) => (
                <option key={index} value={res[1]}>
                  {res[1]}
                </option>
              ))}
          </select>
        </div>

        {/* Frame Rate Selection */}
        <div className="flex flex-col sm:flex-row sm:items-center gap-4">
          <label className="text-lg font-medium">Frame Rate:</label>
          <select
            className="glass p-2 rounded-md w-full text-slate-800"
            value={frameRate}
            onChange={(e) => updateFrameRate(parseInt(e.target.value))}
            disabled={disabled}
          >
            {availableFrameRates &&
              availableFrameRates.map((rate, index) => (
                <option key={index} value={rate}>
                  {rate} FPS
                </option>
              ))}
          </select>
        </div>

        {/* Mouse Cursor Behavior */}
        <div className="flex flex-col sm:flex-row sm:items-center gap-4">
          <label className="text-lg font-medium">Mouse Cursor:</label>
          <select
            className="glass p-2 rounded-md w-full text-slate-800"
            value={pointerBehavior}
            onChange={(e) => updatePointerBehavior(parseInt(e.target.value))}
            disabled={disabled}
          >
            <option value={0}>Hidden</option>
            <option value={1}>System</option>
            <option value={2}>Solid</option>
          </select>
        </div>

        {/* Solid pointer choice */}
        {pointerBehavior >= 2 && (
          <div className="flex flex-col sm:flex-row sm:items-center gap-4">
            <label className="text-lg font-medium">Choose design:</label>
            <Select
              value={pointerBehavior.toString()}
              onValueChange={(value) => updatePointerBehavior(parseInt(value))}
              disabled={disabled}
            >
              <SelectTrigger className="flex items-center glass p-2 rounded-md w-full text-slate-800">
                <SelectValue
                  placeholder={`Solid Pointer ${pointerBehavior - 1}`}
                >
                  <div className="flex items-center gap-2">
                    {renderSolidPointer(pointerBehavior)}
                    <span>Solid Pointer {pointerBehavior - 1}</span>
                  </div>
                </SelectValue>
              </SelectTrigger>
              <SelectContent className="flex items-center p-2 bg-slate-500 w-full">
                {[2, 3, 4, 5].map((value) => (
                  <SelectItem key={value} value={value.toString()}>
                    <div className="flex gap-2 h-5 my-2 w-full">
                      {renderSolidPointer(value)}
                      <span>Solid Pointer {value - 1}</span>
                    </div>
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
        )}

        {/* Recording Controls */}
        <div className="flex justify-center mt-6 gap-4">
          {(!recordingState ||
            recordingState.state === RecordingState.IDLE) && (
            <button
              className="glass px-6 py-3 rounded-lg bg-green-500 text-white hover:bg-green-600 transition"
              onClick={onStartRecording}
            >
              Start Recording
            </button>
          )}
          {recordingState &&
            recordingState.state === RecordingState.RECORDING && (
              <button
                className="glass px-6 py-3 rounded-lg bg-red-500 text-white hover:bg-red-600 transition flex gap-2"
                onClick={onStopRecording}
                title="stop recording"
              >
                <span className="w-full h-full flex items-center justify-center text-lg text-white">
                  {formatElapsedTime(Date.now() - recordingState.instant)}
                </span>
              </button>
            )}
          {recordingState && recordingState.state === RecordingState.DONE && (
            <>
              <button
                className="glass px-6 py-3 rounded-lg bg-red-500 text-white hover:bg-red-600 transition"
                onClick={handleDiscardVideo}
              >
                Discard Video
              </button>
              <button
                className="glass px-6 py-3 rounded-lg bg-green-500 text-white hover:bg-green-600 transition"
                onClick={handleReleaseVideo}
              >
                Release Video
              </button>
            </>
          )}
          {recordingState && recordingState.state === RecordingState.SAVING && (
            <div className="w-full bg-gray-700 rounded-lg p-2 text-center relative h-16">
              <div
                className="absolute left-0 top-0 h-full bg-green-800 rounded-lg overflow-hidden"
                style={{ width: `${recordingState.progress.value * 100}%` }}
              />
              <span className="absolute left-0 top-0 text-white w-full h-full items-center justify-center flex z-10">
                {recordingState.progress.state}
              </span>
            </div>
          )}

          {recordingState &&
            recordingState.state === RecordingState.DISCARDING && (
              <div className="w-full rounded-lg p-2 text-center text-white animate-pulse bg-red-300">
                Discarding...
              </div>
            )}
        </div>
      </div>
      <div className="w-full h-full relative glass p-6 rounded-lg">
        <PastVideosList
          pastVideos={pastVideos}
          refreshPastVideos={refreshPastVideos}
          removeRecording={(index) =>
            console.log("Remove recording at index", index)
          }
        />
      </div>
    </div>
  );
}

const formatDuration = (seconds) => {
  const mins = Math.floor(seconds / 60);
  const secs = seconds % 60;
  return `${String(mins).padStart(2, "0")}:${String(secs).padStart(2, "0")}`;
};

const formatDate = (secsSinceEpoch) => {
  const date = new Date(secsSinceEpoch * 1000);
  return date.toLocaleString(); // Adjust format as needed
};

const getFilename = (filePath) => filePath.split("/").pop();

const PastVideosList = ({ pastVideos, removeRecording }) => {
  return (
    <>
      <h2 className="text-xl font-semibold">Recordings</h2>
      <div className="absolute overflow-y-auto h-[calc(100%-72px)] w-full left-0 top-16 p-6 rounded-lg">
        <ul className="space-y-3">
          {pastVideos &&
            [...pastVideos].reverse().map((rec, index) => (
              <li
                key={index}
                className="glass p-3 rounded-md flex justify-between items-center"
              >
                <div className="flex flex-col cursor-pointer items-start">
                  <span className="font-medium">
                    {getFilename(rec.file_path)}
                  </span>
                  <span className="text-sm text-gray-300">
                    {formatDuration(rec.duration)}
                  </span>
                  <span className="text-sm text-gray-200">
                    {formatDate(rec.time_recorded)}
                  </span>
                </div>
                <X
                  className="h-5 w-5 text-red-500 cursor-pointer"
                  onClick={() => removeRecording(index)}
                />
              </li>
            ))}
        </ul>
      </div>
    </>
  );
};

async function getRecordingState() {
  const state = await invoke("recording_state");

  if (state === RecordingState.IDLE) {
    return { state: RecordingState.IDLE };
  }

  if (typeof state === "object" && RecordingState.RECORDING in state) {
    return { state: RecordingState.RECORDING, instant: state.Recording };
  }

  if (typeof state === "object" && RecordingState.DONE in state) {
    return { state: RecordingState.DONE, duration: state.Done };
  }

  throw new Error("Unknown RecordingState received from backend");
}

async function getSavingStateAsRecordingState() {
  const state = await invoke("saving_progress");

  if (state === SavingState.INITIALIZING) {
    return {
      state: RecordingState.SAVING,
      progress: { state: SavingState.INITIALIZING, value: 0 },
    };
  }

  if (typeof state === "object" && SavingState.SAVING in state) {
    return {
      state: RecordingState.SAVING,
      progress: {
        state: SavingState.SAVING,
        value: state.Saving[0] / state.Saving[1],
      },
    };
  }

  if (state === SavingState.FINALIZING) {
    return {
      state: RecordingState.SAVING,
      progress: { state: SavingState.FINALIZING, value: 1 },
    };
  }

  if (state === SavingState.DONE) {
    return {
      state: RecordingState.SAVING,
      progress: { state: SavingState.DONE, value: 1 },
    };
  }

  throw new Error("Unknown SavingState received from backend");
}

function formatElapsedTime(ms) {
  const totalSeconds = Math.floor(ms / 1000);
  const seconds = totalSeconds % 60;
  const totalMinutes = Math.floor(totalSeconds / 60);
  const minutes = totalMinutes % 60;
  const totalHours = Math.floor(totalMinutes / 60);
  const hours = totalHours % 24;
  const days = Math.floor(totalHours / 24);

  if (days > 0) {
    return `${String(days).padStart(2, "0")}:${String(hours).padStart(2, "0")} days`;
  } else if (totalHours > 0) {
    return `${String(totalHours).padStart(2, "0")}:${String(minutes).padStart(2, "0")} hours`;
  } else if (totalMinutes > 0) {
    return `${String(totalMinutes).padStart(2, "0")}:${String(seconds).padStart(2, "0")} minutes`;
  } else {
    return `${String(seconds).padStart(2, "0")}:${String(ms % 1000).padStart(3, "0")} seconds`;
  }
}

export default RecorderMode;
