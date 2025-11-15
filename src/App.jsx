import React, { useState } from "react";
import { RecorderProvider } from "./context/RecorderContext";
import { RecorderMode } from "./components/RecorderMode";
import { SettingsMode } from "./components/SettingsMode";
import NavigationTabs from "./components/NavigationTabs";
import "./App.css";

function App() {
  const [mode, setMode] = useState("recorder"); // recorder | settings

  return (
    <RecorderProvider>
      <main className="app-container">
        <div className="app-content">
          <NavigationTabs mode={mode} setMode={setMode} />
          
          <div className="mode-container">
            {mode === "recorder" && <RecorderMode />}
            {mode === "settings" && <SettingsMode />}
          </div>
        </div>
      </main>
    </RecorderProvider>
  );
}

export default App;
