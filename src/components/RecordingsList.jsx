import React, { useEffect, useState } from "react";

function RecordingsList({ onViewRecording }) {
    const [recordings, setRecordings] = useState([]);
    const [loading, setLoading] = useState(true);
    
    useEffect(() => {
        // Fetch recordings from Tauri
        setLoading(false);
    }, []);

    return (
        <div className="recordings-list">
            <h2>Previous Recordings</h2>
            {recordings.length === 0 ? (
                <p>No recordings available.</p>
            ) : (
                <ul>
                    {recordings.map((rec, index) => (
                        <li key={index} onClick={() => onViewRecording(rec)}>
                            {rec}
                        </li>
                    ))}
                </ul>
            )}
        </div>
    );
}

export default RecordingsList;
