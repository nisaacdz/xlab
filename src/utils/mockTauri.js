// Mock Tauri API for frontend development
let mockRecordingState = { state: "Idle" };
let mockPastVideos = [
  {
    file_path: "/path/to/video1.mp4",
    duration: 125, // seconds
    time_recorded: Math.floor(Date.now() / 1000) - 86400, // 1 day ago
  },
  {
    file_path: "/path/to/video2.mp4",
    duration: 300,
    time_recorded: Math.floor(Date.now() / 1000) - 172800, // 2 days ago
  },
  {
    file_path: "/path/to/recording_2025_01_01.mp4",
    duration: 75,
    time_recorded: Math.floor(Date.now() / 1000) - 259200, // 3 days ago
  },
];

let recordingStartTime = null;
let mockSavingState = null;

export const mockInvoke = async (command, args) => {
  // Simulate network delay
  await new Promise((resolve) => setTimeout(resolve, 100));

  switch (command) {
    case "recording_state":
      return mockRecordingState;

    case "past_videos":
      return mockPastVideos;

    case "start_recording":
      mockRecordingState = {
        state: "Recording",
        instant: new Date().toISOString(),
      };
      recordingStartTime = Date.now();
      return null;

    case "stop_recording":
      mockRecordingState = {
        state: "Done",
        duration: Math.floor((Date.now() - recordingStartTime) / 1000),
      };
      return null;

    case "save_recording":
      mockSavingState = "Initializing";
      // Simulate saving process
      setTimeout(() => {
        mockSavingState = "Done";
        mockRecordingState = { state: "Idle" };
      }, 2000);
      return null;

    case "discard_recording":
      mockRecordingState = { state: "Idle" };
      return null;

    case "saving_progress":
      return mockSavingState;

    case "open_file_location":
      console.log("Mock: Opening file location:", args.path);
      return null;

    case "remove_previous_recording_by_index":
      mockPastVideos.splice(args.index, 1);
      return null;

    default:
      console.warn("Mock invoke called with unknown command:", command);
      return null;
  }
};

// Setup mock in development
export const setupMockTauri = () => {
  if (typeof window !== "undefined" && !window.__TAURI__) {
    console.log("Setting up mock Tauri API for development");
    window.__TAURI_INVOKE__ = mockInvoke;
  }
};
