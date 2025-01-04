import React, { useState } from "react";
import EditorMode from './components/EditorMode';
import RecorderMode from './components/RecorderMode';
import RecordingsList from './components/RecordingsList';
import "./App.css";

function App() {
    const [mode, setMode] = useState('recorder'); // recorder | editor
    const [currentRecording, setCurrentRecording] = useState(null);

    const handleViewRecording = (recording) => {
        setCurrentRecording(recording);
        setMode('editor');
    };

    const handleBackToIdle = () => {
        setCurrentRecording(null);
        setMode('recorder');
    };

    return (
        <main className="app-container flex flex-col w-full h-full">
            <h1 className="text-2xl font-bold text-center text-transparent bg-clip-text bg-gradient-to-r from-blue-500 to-purple-500 mb-6">
                Screen Recorder
            </h1>
            <div className="px-8 py-4 grid grid-cols-2 w-full h-full">
                {mode === 'recorder' && (
                    <RecorderMode />
                )}
                {mode === 'editor' && (
                    <EditorMode />
                )}
                <RecordingsList onViewRecording={handleViewRecording} />
            </div>
        </main>
    );
}

export default App;