// Mock Tauri API for frontend development
let mockRecordingState = "Idle";
let mockPastVideos = [
  {
    file_path: "/path/to/video1.mp4",
    duration: 125, // seconds
    time_recorded: { secs_since_epoch: Math.floor(Date.now() / 1000) - 86400 }, // 1 day ago
  },
  {
    file_path: "/path/to/video2.mp4",
    duration: 300,
    time_recorded: { secs_since_epoch: Math.floor(Date.now() / 1000) - 172800 }, // 2 days ago
  },
  {
    file_path: "/path/to/recording_2025_01_01.mp4",
    duration: 75,
    time_recorded: { secs_since_epoch: Math.floor(Date.now() / 1000) - 259200 }, // 3 days ago
  },
];

const mockAvailableResolutions = [
  [1920, 1080],
  [1280, 720],
  [854, 480],
  [640, 360],
];

const mockAvailableFrameRates = [60, 30, 24, 15];

let recordingStartTime = null;
let mockSavingState = null;
let mockCurrentFrameRate = 30;
let mockCurrentResolutionIndex = 1;
let mockCurrentPointer = 1;

export const mockInvoke = async (command, args) => {
  // Simulate network delay
  await new Promise((resolve) => setTimeout(resolve, 100));

  switch (command) {
    case "recording_state":
      return mockRecordingState;

    case "past_videos":
      return mockPastVideos;

    case "available_resolutions":
      return mockAvailableResolutions;

    case "available_frame_rates":
      return mockAvailableFrameRates;

    case "update_frame_rate":
      mockCurrentFrameRate = args.frameRate;
      console.log("Mock: Updated frame rate to", mockCurrentFrameRate);
      return null;

    case "update_resolution":
      mockCurrentResolutionIndex = args.index;
      console.log("Mock: Updated resolution to", mockAvailableResolutions[args.index]);
      return null;

    case "update_pointer":
      mockCurrentPointer = args.index;
      console.log("Mock: Updated pointer to", args.index);
      return null;

    case "start_recording":
      mockRecordingState = { Recording: Date.now() };
      recordingStartTime = Date.now();
      return null;

    case "stop_recording":
      mockRecordingState = { Done: Math.floor((Date.now() - recordingStartTime) / 1000) };
      return null;

    case "save_recording":
      mockSavingState = "Initializing";
      // Simulate saving process
      setTimeout(() => {
        mockSavingState = "Done";
        mockRecordingState = "Idle";
      }, 2000);
      return null;

    case "discard_recording":
      mockRecordingState = "Idle";
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
