import React, { useState } from 'react';

const CursorKinds = {
    Hidden: 'hidden',
    Solid: 'solid',
    System: 'system',
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
        <div className="flex flex-col items-center justify-center min-h-screen bg-gray-100">
            <div></div>
            <div className="selector">
                <label className="block text-gray-700 font-medium mb-1">Resolution</label>
                <select
                    className="w-full bg-gray-100 rounded-lg p-2 border border-gray-300 focus:outline-none focus:ring focus:ring-blue-500"
                    onChange={(e) => setResolution(e.target.value.split('x').map(Number))}
                    disabled={isRecording}
                >
                    {RESOLUTIONS.map((res, index) => (
                        <option key={index} value={res.join('x')}>
                            {res.join('x')}
                        </option>
                    ))}
                </select>
            </div>
            <div className="selector">
                <label className="block text-gray-700 font-medium mb-1">Frame Rate</label>
                <select
                    className="w-full bg-gray-100 rounded-lg p-2 border border-gray-300 focus:outline-none focus:ring focus:ring-blue-500"
                    value={frameRate}
                    onChange={(e) => setFrameRate(parseInt(e.target.value))}
                    disabled={isRecording}
                >
                    {FRAME_RATES.map((rate, index) => (
                        <option key={index} value={rate}>
                            {rate}
                        </option>
                    ))}
                </select>
            </div>
            <div className="selector">
                <label className="block text-gray-700 font-medium mb-1">Mouse Cursor Behaviour</label>
                <select
                    className="w-full bg-gray-100 rounded-lg p-2 border border-gray-300 focus:outline-none focus:ring focus:ring-blue-500"
                    value={pointerBehaviourType.type}
                    onChange={(e) =>
                        setPointerBehaviourType({ type: e.target.value })
                    }
                    disabled={isRecording}
                >
                    <option value={CursorKinds.Hidden}>Hidden</option>
                    <option value={CursorKinds.System}>System</option>
                    <option value={CursorKinds.Solid}>Solid</option>
                </select>
                {pointerBehaviourType.type === CursorKinds.Solid && (
                    <select
                        className="w-full bg-gray-100 rounded-lg p-2 mt-2 border border-gray-300 focus:outline-none focus:ring focus:ring-blue-500"
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
                )}
            </div>
            <div className="mt-6">
                {!isRecording ? (
                    <button
                        className="w-full bg-blue-500 text-white py-2 rounded-lg shadow hover:bg-blue-600 transition duration-200"
                        onClick={onStartRecording}
                    >
                        Start Recording
                    </button>
                ) : (
                    <button
                        className="w-full bg-red-500 text-white py-2 rounded-lg shadow hover:bg-red-600 transition duration-200"
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
