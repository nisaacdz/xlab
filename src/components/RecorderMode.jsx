import { PlusCircleIcon, XMarkIcon } from "@heroicons/react/24/outline";
import React, { useState } from "react";
import { useRecorder } from "../context/RecorderContext";

function RecorderMode() {
    const [isRecording, setIsRecording] = useState(false);
    const {
        resolution,
        setResolution,
        frameRate,
        setFrameRate,
        pointerBehavior,
        setPointerBehavior,
        availableResolutions,
        availableFrameRates,
        pastVideos,
        refreshPastVideos,
    } = useRecorder();

    const onStartRecording = () => setIsRecording(true);
    const onStopRecording = () => setIsRecording(false);

    return (
        <div className="grid grid-cols-2 w-full h-full gap-6">
            <div className="glass w-full h-full p-6 rounded-lg grid grid-cols-1 gap-6">
                <h2 className="text-xl font-semibold text-center">Recorder Settings</h2>

                {/* Resolution Selection */}
                <div className="flex flex-col sm:flex-row sm:items-center gap-4">
                    <label className="text-lg font-medium">Resolution:</label>
                    <select
                        className="glass p-2 rounded-md w-full"
                        onChange={(e) => setResolution(e.target.value.split(',').map(Number))}
                        disabled={isRecording || !availableResolutions}
                    >
                        {availableResolutions && availableResolutions.map((res, index) => (
                            <option key={index} value={res.toString()} selected={res[0] === resolution[0] && res[1] === resolution[1]}>
                                {res[1]}
                            </option>
                        ))}
                        {!availableResolutions && <option value={resolution.toString()} selected={true}>{resolution[1]}</option>}
                    </select>
                </div>

                {/* Frame Rate Selection */}
                <div className="flex flex-col sm:flex-row sm:items-center gap-4">
                    <label className="text-lg font-medium">Frame Rate:</label>
                    <select
                        className="glass p-2 rounded-md w-full"
                        onChange={(e) => setFrameRate(parseInt(e.target.value))}
                        disabled={isRecording || !availableFrameRates}
                    >
                        {availableFrameRates && availableFrameRates.map((rate, index) => (
                            <option key={index} value={rate} selected={rate == frameRate}>
                                {rate} FPS
                            </option>
                        ))}
                        {!availableResolutions && <option value={frameRate} selected={true}>
                            {frameRate} FPS
                        </option>}
                    </select>
                </div>

                {/* Mouse Cursor Behavior */}
                <div className="flex flex-col sm:flex-row sm:items-center gap-4">
                    <label className="text-lg font-medium">Mouse Cursor:</label>
                    <select
                        className="glass p-2 rounded-md w-full"
                        onChange={(e) => setPointerBehavior(parseInt(e.target.value))}
                        disabled={isRecording}
                    >
                        <option value={0} selected={pointerBehavior === 0}>Hidden</option>
                        <option value={1} selected={pointerBehavior === 1}>System</option>
                        <option value={2} selected={pointerBehavior > 1}>Solid</option>
                    </select>
                </div>

                {/* Solid Pointer Selection */}
                {pointerBehavior > 1 && (
                    <div className="flex flex-col sm:flex-row sm:items-center gap-4">
                        <label className="text-lg font-medium">Pointer Style:</label>
                        <select
                            className="glass p-2 rounded-md w-full"
                            onChange={(e) => setPointerBehavior(parseInt(e.target.value))}
                            disabled={isRecording}
                        >
                            {SOLID_POINTERS.map((pointer, index) => (
                                <option key={index} value={index}>
                                    <img src={`../assets/pointer_${pointer}`} alt="Pointer" className="h-6 w-6" />
                                </option>
                            ))}
                        </select>
                    </div>
                )}

                {/* Recording Controls */}
                <div className="flex justify-center mt-6">
                    {!isRecording ? (
                        <button
                            className="glass px-6 py-3 rounded-lg bg-green-500 text-white hover:bg-green-600 transition"
                            onClick={onStartRecording}
                        >
                            Start Recording
                        </button>
                    ) : (
                        <button
                            className="glass px-6 py-3 rounded-lg bg-red-500 text-white hover:bg-red-600 transition"
                            onClick={onStopRecording}
                        >
                            Stop Recording
                        </button>
                    )}
                </div>
            </div>
            <div className="glass w-full h-full p-6 rounded-lg">
                <h2 className="text-xl font-semibold mb-4">Recordings</h2>

                <ul className="space-y-3">
                    {pastVideos && pastVideos.map((rec, index) => (
                        <li key={index} className="glass p-3 rounded-md flex justify-between items-center">
                            <span className="cursor-pointer" onClick={() => onViewRecording(rec)}>{rec}</span>
                            <XMarkIcon className="h-5 w-5 text-red-500 cursor-pointer" onClick={() => removeRecording(index)} />
                        </li>
                    ))}
                </ul>

                <button className="glass w-full mt-6 p-3 flex items-center justify-center rounded-md">
                    <PlusCircleIcon className="h-6 w-6 mr-2" /> Import Video
                </button>
            </div>
        </div>
    );
}

export default RecorderMode;
