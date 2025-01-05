import React, { useState } from "react";
import { TrashIcon, PlusCircleIcon } from "@heroicons/react/24/outline";

function RecordingsList({ onViewRecording }) {
    const [recordings, setRecordings] = useState(["Video1.mp4", "Video2.mp4"]);

    const removeRecording = (index) => {
        setRecordings(recordings.filter((_, i) => i !== index));
    };

    return (
        <div className="glass w-full h-full p-6 rounded-lg">
            <h2 className="text-xl font-semibold mb-4">Recordings</h2>

            <ul className="space-y-3">
                {recordings.map((rec, index) => (
                    <li key={index} className="glass p-3 rounded-md flex justify-between items-center">
                        <span className="cursor-pointer" onClick={() => onViewRecording(rec)}>{rec}</span>
                        <TrashIcon className="h-5 w-5 text-red-500 cursor-pointer" onClick={() => removeRecording(index)} />
                    </li>
                ))}
            </ul>

            <button className="glass w-full mt-6 p-3 flex items-center justify-center rounded-md">
                <PlusCircleIcon className="h-6 w-6 mr-2" /> Import Video
            </button>
        </div>
    );
}

export default RecordingsList;
