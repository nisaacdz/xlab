import React from "react";
import { X } from "lucide-react";
import { formatDuration, formatDate, getFilename } from "../utils/formatters";

function PastVideosList({ pastVideos, removeRecording }) {
  const reversedVideos = pastVideos ? [...pastVideos].reverse() : [];

  return (
    <>
      <h2 className="text-xl font-semibold">Recordings</h2>
      <div className="absolute overflow-y-auto h-[calc(100%-72px)] w-full left-0 top-16 p-6 rounded-lg">
        <ul className="space-y-3">
          {reversedVideos.map((rec, index) => (
            <li
              key={index}
              className="glass p-2 rounded-md flex justify-between items-center"
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
                title="Remove recording from list"
                onClick={() =>
                  removeRecording(reversedVideos.length - index - 1)
                }
              />
            </li>
          ))}
        </ul>
      </div>
    </>
  );
}

export default PastVideosList;
