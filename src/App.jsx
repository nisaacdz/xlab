import React, { useState } from "react";
import { RecorderProvider } from "./context/RecorderContext";
import EditorMode from "./components/EditorMode";
import RecorderMode from "./components/RecorderMode";
import SettingsMode from "./components/SettingsMode";
import NavigationTabs from "./components/NavigationTabs";
import "./App.css";

function App() {
  const [mode, setMode] = useState("recorder");

  return (
    <RecorderProvider>
      <main className="app-container flex flex-col items-center w-full h-screen p-6 bg-gray-900 text-white">
        <NavigationTabs mode={mode} setMode={setMode} />
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
