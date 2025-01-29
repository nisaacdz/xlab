import React from "react";
import {
  Select,
  SelectTrigger,
  SelectValue,
  SelectContent,
  SelectItem,
} from "@/components/ui/select";
import renderSolidPointer from "./SolidPointers";

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
    <div className="grid grid-cols-1 md:grid-cols-3 gap-0 md:gap-4">
      {/* Resolution Selection */}
      <label className="text-lg font-medium text-left md:text-right">
        Resolution:
      </label>
      <select
        className="glass p-2 rounded-md w-full text-slate-800 cursor-pointer mb-4 md:mb-0 col-span-2"
        value={resolution[1]}
        onChange={(e) => updateResolution(e.target.selectedIndex)}
        disabled={disabled}
      >
        {availableResolutions?.map((res, index) => (
          <option key={index} value={res[1]} className="cursor-pointer">
            {res[1]}
          </option>
        ))}
      </select>

      {/* Frame Rate Selection */}
      <label className="text-lg font-medium text-left md:text-right">
        Frame Rate:
      </label>
      <select
        className="glass p-2 rounded-md w-full text-slate-800 cursor-pointer mb-4 md:mb-0 col-span-2"
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

      {/* Mouse Cursor Behavior */}
      <label className="text-lg font-medium text-left md:text-right">
        Mouse Cursor:
      </label>
      <select
        className="glass p-2 rounded-md w-full text-slate-800 cursor-pointer mb-4 md:mb-0 col-span-2"
        value={pointerBehavior}
        onChange={(e) => updatePointerBehavior(parseInt(e.target.value))}
        disabled={disabled}
      >
        <option value={0}>Hidden</option>
        <option value={1}>System</option>
        <option value={2}>Solid</option>
      </select>

      {/* Solid Pointer Choice */}
      {pointerBehavior >= 2 && (
        <>
          <label className="text-lg font-medium text-left md:text-right">
            Choose design:
          </label>
          <Select
            value={pointerBehavior.toString()}
            onValueChange={(value) => updatePointerBehavior(parseInt(value))}
            disabled={disabled}
          >
            <SelectTrigger className="glass p-2 rounded-md w-full text-slate-800 cursor-pointer col-span-2">
              <SelectValue placeholder={`Solid Pointer ${pointerBehavior - 1}`}>
                <div className="flex gap-2 h-5 my-2 w-full">
                  {renderSolidPointer(pointerBehavior)}
                  <span>Solid Pointer {pointerBehavior - 1}</span>
                </div>
              </SelectValue>
            </SelectTrigger>
            <SelectContent className="flex items-center p-2 bg-slate-500 pr-6">
              {[2, 3, 4, 5].map((value) => (
                <SelectItem key={value} value={value.toString()}>
                  <div className="flex gap-2 h-5 my-2 w-full cursor-pointer">
                    {renderSolidPointer(value)}
                    <span>Solid Pointer {value - 1}</span>
                  </div>
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </>
      )}
    </div>
  );
}

export default RecordingOptions;
