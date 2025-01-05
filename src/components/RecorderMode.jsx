import React, { useState } from "react";
import { CursorArrowRaysIcon, EyeSlashIcon, CursorArrowRippleIcon } from "@heroicons/react/24/outline";

const CursorKinds = {
    Hidden: "hidden",
    Solid: "solid",
    System: "system",
};

const FRAME_RATES = [15, 24, 30, 60, 90];
const RESOLUTIONS = [
    [1920, 1080],
    [1280, 720],
    [854, 480],
    [640, 360],
    [426, 240],
];

const SOLID_POINTERS = [{ id: 1 }, { id: 2 }, { id: 3 }, { id: 4 }];

function RecorderMode() {
    const [isRecording, setIsRecording] = useState(false);
    const [resolution, setResolution] = useState([1920, 1080]);
    const [frameRate, setFrameRate] = useState(30);
    const [pointerBehaviourType, setPointerBehaviourType] = useState({ type: CursorKinds.System });

    const onStartRecording = () => setIsRecording(true);
    const onStopRecording = () => setIsRecording(false);

    return (
        <div className="glass w-full h-full p-6 rounded-lg grid grid-cols-1 gap-6">
            <h2 className="text-xl font-semibold text-center">Recorder Settings</h2>

            {/* Resolution Selection */}
            <div className="flex flex-col sm:flex-row sm:items-center gap-4">
                <label className="text-lg font-medium">Resolution:</label>
                <select
                    className="glass p-2 rounded-md w-full"
                    onChange={(e) => setResolution(e.target.value.split("x").map(Number))}
                    disabled={isRecording}
                >
                    {RESOLUTIONS.map((res, index) => (
                        <option key={index} value={res.join("x")}>
                            {res.join(" x ")}
                        </option>
                    ))}
                </select>
            </div>

            {/* Frame Rate Selection */}
            <div className="flex flex-col sm:flex-row sm:items-center gap-4">
                <label className="text-lg font-medium">Frame Rate:</label>
                <select
                    className="glass p-2 rounded-md w-full"
                    value={frameRate}
                    onChange={(e) => setFrameRate(parseInt(e.target.value))}
                    disabled={isRecording}
                >
                    {FRAME_RATES.map((rate, index) => (
                        <option key={index} value={rate}>
                            {rate} FPS
                        </option>
                    ))}
                </select>
            </div>

            {/* Mouse Cursor Behavior */}
            <div className="flex flex-col sm:flex-row sm:items-center gap-4">
                <label className="text-lg font-medium">Mouse Cursor:</label>
                <select
                    className="glass p-2 rounded-md w-full"
                    value={pointerBehaviourType.type}
                    onChange={(e) => setPointerBehaviourType({ type: e.target.value })}
                    disabled={isRecording}
                >
                    <option value={CursorKinds.Hidden}>Hidden</option>
                    <option value={CursorKinds.System}>System</option>
                    <option value={CursorKinds.Solid}>Solid</option>
                </select>
            </div>

            {/* Solid Pointer Selection */}
            {pointerBehaviourType.type === CursorKinds.Solid && (
                <div className="flex flex-col sm:flex-row sm:items-center gap-4">
                    <label className="text-lg font-medium">Pointer Style:</label>
                    <select
                        className="glass p-2 rounded-md w-full"
                        onChange={(e) =>
                            setPointerBehaviourType({
                                type: CursorKinds.Solid,
                                pointer: SOLID_POINTERS[parseInt(e.target.value)],
                            })
                        }
                        disabled={isRecording}
                    >
                        {SOLID_POINTERS.map((pointer, index) => (
                            <option key={index} value={index}>
                                Pointer {pointer.id}
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
    );
}

export default RecorderMode;
