import React from "react";
import { Folder, Trash2 } from "lucide-react";
import { formatDuration, formatDate, getFilename } from "../utils/formatters";

function PastRecordings({ pastVideos, removeRecording, openVideoInExplorer }) {
  const reversedVideos = pastVideos ? [...pastVideos].reverse() : [];

  return (
    <>
      <h2 className="text-xl font-semibold mb-4">Your Recordings</h2>
      {reversedVideos.length === 0 ? (
        <div className="flex items-center justify-center h-[calc(100%-40px)]">
          <p className="text-gray-400">No recordings yet.</p>
        </div>
      ) : (
        <div className="absolute overflow-y-auto h-[calc(100%-72px)] w-full left-0 top-16 p-6 rounded-lg">
          <ul className="space-y-4">
            {reversedVideos.map((rec, index) => (
              <li
                key={index}
                className="glass p-4 rounded-lg flex justify-between items-center transition-all hover:bg-gray-700/50"
              >
                <div className="flex flex-col items-start">
                  <span className="font-bold text-lg">
                    {getFilename(rec.file_path)}
                  </span>
                  <div className="flex items-center gap-4 text-sm text-gray-300 mt-1">
                    <span>{formatDuration(rec.duration)}</span>
                    <span>{formatDate(rec.time_recorded)}</span>
                  </div>
                </div>
                <div className="flex items-center gap-2">
                  <button
                    className="p-2 rounded-full hover:bg-gray-600 transition-colors"
                    title="Open file location"
                    onClick={() =>
                      openVideoInExplorer(reversedVideos.length - index - 1)
                    }
                  >
                    <Folder className="h-5 w-5 text-blue-400" />
                  </button>
                  <button
                    className="p-2 rounded-full hover:bg-gray-600 transition-colors"
                    title="Remove recording from list"
                    onClick={() =>
                      removeRecording(reversedVideos.length - index - 1)
                    }
                  >
                    <Trash2 className="h-5 w-5 text-red-500" />
                  </button>
                </div>
              </li>
            ))}
          </ul>
        </div>
      )}
    </>
  );
}

export default PastRecordings;
