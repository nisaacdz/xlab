import React from "react";

function ProgressBar({ progress }) {
  return (
    <div className="w-full bg-gray-700 rounded-lg p-2 text-center relative h-16">
      <div
        className="absolute left-0 top-0 h-full bg-green-800 rounded-lg overflow-hidden"
        style={{ width: `${progress.value * 100}%` }}
      />
      <span className="absolute left-0 top-0 text-white w-full h-full items-center justify-center flex z-10">
        {progress.state}
      </span>
    </div>
  );
}

export default ProgressBar;
