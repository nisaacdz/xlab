import React from "react";
import { PointerPreview } from "./PointerPreview";

function RecordingOptions({
  resolution,
  updateResolution,
  frameRate,
  updateFrameRate,
  pointerBehavior,
  updatePointerBehavior,
  availableResolutions,
  availableFrameRates,
  disabled,
}) {
  return (
    <div className="flex flex-col gap-6">
      {/* Resolution Selection */}
      <div className="flex flex-col gap-3">
        <label className="text-lg font-medium">Resolution:</label>
        <select
          className="glass p-3 rounded-lg w-full cursor-pointer"
          value={resolution ? resolution[1] : ""}
          onChange={(e) => updateResolution(e.target.selectedIndex)}
          disabled={disabled}
        >
          {availableResolutions?.map((res, index) => (
            <option key={index} value={res[1]}>
              {res[1]}
            </option>
          ))}
        </select>
      </div>

      {/* Frame Rate Selection */}
      <div className="flex flex-col gap-3">
        <label className="text-lg font-medium">Frame Rate:</label>
        <select
          className="glass p-3 rounded-lg w-full cursor-pointer"
          value={frameRate}
          onChange={(e) => updateFrameRate(parseInt(e.target.value))}
          disabled={disabled}
        >
          {availableFrameRates?.map((rate, index) => (
            <option key={index} value={rate}>
              {rate} FPS
            </option>
          ))}
        </select>
      </div>

      {/* Pointer Preview with Visual Selection */}
      <PointerPreview
        currentPointer={pointerBehavior}
        onSelect={updatePointerBehavior}
        disabled={disabled}
      />
    </div>
  );
}

export default RecordingOptions;
