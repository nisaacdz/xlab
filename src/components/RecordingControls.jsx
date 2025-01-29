import React from "react";
import { formatTime } from "../utils/formatters";
import ProgressBar from "./ProgressBar";
import { RecordingState } from "../utils/api";

function RecordingControls({
  recordingState,
  onStartRecording,
  onStopRecording,
  onDiscardRecording,
  onReleaseRecording,
}) {
  return (
    <div className="flex justify-center mt-6 gap-4">
      {(!recordingState || recordingState?.state === RecordingState.IDLE) && (
        <button
          className="glass px-6 py-3 rounded-lg bg-green-500 text-white hover:bg-green-600 transition"
          onClick={onStartRecording}
        >
          Start Recording
        </button>
      )}
      {recordingState?.state === RecordingState.RECORDING && (
        <button
          className="glass px-6 py-3 rounded-lg bg-red-500 text-white hover:bg-red-600 transition flex gap-2"
          onClick={onStopRecording}
          title="Stop recording"
        >
          <span className="w-full h-full flex items-center justify-center text-lg text-white">
            {formatTime(Date.now() - new Date(recordingState.instant))}
          </span>
        </button>
      )}
      {recordingState?.state === RecordingState.DONE && (
        <>
          <button
            className="glass px-6 py-3 rounded-lg bg-red-500 text-white hover:bg-red-600 transition"
            onClick={onDiscardRecording}
          >
            Discard Video
          </button>
          <button
            className="glass px-6 py-3 rounded-lg bg-green-500 text-white hover:bg-green-600 transition"
            onClick={onReleaseRecording}
          >
            Release Video
          </button>
        </>
      )}
      {recordingState?.state === RecordingState.SAVING && (
        <ProgressBar progress={recordingState.progress} />
      )}
    </div>
  );
}

export default RecordingControls;
