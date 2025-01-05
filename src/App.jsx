import React, { useState } from "react";
import EditorMode from "./components/EditorMode";
import RecorderMode from "./components/RecorderMode";
import RecordingsList from "./components/RecordingsList";
import { ArrowPathIcon } from "@heroicons/react/24/outline"; // ✅ Correct alternative in v2
import "./App.css";

function App() {
    const [mode, setMode] = useState("recorder"); // recorder | editor
    const [currentRecording, setCurrentRecording] = useState(null);

    const handleViewRecording = (recording) => {
        setCurrentRecording(recording);
        setMode("editor");
    };

    const handleBackToIdle = () => {
        setCurrentRecording(null);
        setMode("recorder");
    };

    return (
        <main className="app-container flex flex-col w-full h-screen p-6 bg-gray-900 text-white">
            {/* Header */}
            <header className="glass w-full p-4 rounded-lg text-center text-lg font-semibold">
                Screen Recorder
            </header>

            {/* Main Content */}
            <div className={`flex w-full h-full mt-6 transition-all ${mode === "editor" ? "grid grid-cols-1" : "grid grid-cols-2 gap-4"}`}>
                {mode === "recorder" && <RecorderMode />}
                {mode === "editor" && <EditorMode />}
                <RecordingsList onViewRecording={handleViewRecording} />
            </div>

            {/* Mode Switch Button */}
            <button
                onClick={handleBackToIdle}
                className="fixed bottom-6 right-6 glass p-3 rounded-full shadow-lg transition hover:scale-105"
            >
                <ArrowPathIcon className="h-6 w-6 text-white" />
            </button>
        </main>
    );
}

export default App;
