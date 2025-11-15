import { invoke } from "@tauri-apps/api/core";

export const RecordingState = Object.freeze({
  IDLE: "Idle",
  RECORDING: "Recording",
  DONE: "Done",
  SAVING: "Saving",
});

export const SavingState = Object.freeze({
  INITIALIZING: "Initializing",
  SAVING: "Saving",
  FINALIZING: "Finalizing",
  DONE: "Done",
});

export async function getRecordingState() {
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

export async function getSavingStateAsRecordingState() {
  const state = await invoke("saving_progress");

  if (state === null) {
    return null;
  }

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

export async function startRecording() {
  await invoke("start_recording");
}

export async function stopRecording() {
  await invoke("stop_recording");
}

export async function discardRecording() {
  await invoke("discard_recording");
}

export async function releaseRecording() {
  await invoke("save_recording");
}

export async function captureCurrentPointerImage() {
  return await invoke("capture_current_pointer_image");
}
