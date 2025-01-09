// App.jsx
import React, { useState } from "react";
import { RecorderProvider } from "./context/RecorderContext";
import EditorMode from "./components/EditorMode";
import RecorderMode from "./components/RecorderMode";
import { ArrowPathIcon } from "@heroicons/react/24/outline";
import recorderIcon from "./assets/recorder.svg";
import editorIcon from "./assets/editor.svg";
import settingsIcon from "./assets/settings.svg";
import "./App.css";
import SettingsMode from "./components/SettingsMode";

function App() {
    const [mode, setMode] = useState("recorder"); // recorder | editor | settings
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
        <RecorderProvider>
            <main className="app-container flex flex-col items-center w-full h-screen p-6 bg-gray-900 text-white">
                {/* Navigation Tabs */}
                <div className="flex justify-evenly bg-gray-800 p-3 rounded-lg max-w-96 min-w-64">
                    <button onClick={() => setMode("recorder")} className={`p-2 ${mode !== "recorder" ? "bg-gray-700 rounded-md" : ""}`}>
                        <img src={recorderIcon} alt="Recorder" className="h-6 w-6" />
                    </button>
                    <button onClick={() => setMode("editor")} className={`p-2 ${mode !== "editor" ? "bg-gray-700 rounded-md" : ""}`}>
                        <img src={editorIcon} alt="Editor" className="h-6 w-6" />
                    </button>
                    <button onClick={() => setMode("settings")} className={`p-2 ${mode !== "settings" ? "bg-gray-700 rounded-md" : ""}`}>
                        <img src={settingsIcon} alt="Settings" className="h-6 w-6" />
                    </button>
                </div>

                {/* Main Content */}
                <div className="flex w-full h-full mt-6 transition-all">
                    {mode === "recorder" && <RecorderMode />}
                    {mode === "editor" && <EditorMode />}
                    {mode === "settings" && <SettingsMode />}
                </div>
            </main>
        </RecorderProvider>
    );
}

export default App;
