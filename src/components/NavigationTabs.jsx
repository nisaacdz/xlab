import React from "react";
import recorderIcon from "../assets/recorder.svg";
import editorIcon from "../assets/editor.svg";
import settingsIcon from "../assets/settings.svg";

const modes = [
  { id: "recorder", icon: recorderIcon, alt: "Recorder", enabled: true },
  { id: "editor", icon: editorIcon, alt: "Editor", enabled: false },
  { id: "settings", icon: settingsIcon, alt: "Settings", enabled: false },
];
// Enable only the recorder tab for now
// until I come up ideas for the other tabs

function NavigationTabs({ mode, setMode }) {
  return (
    <div className="flex justify-evenly bg-gray-800 p-3 rounded-lg max-w-96 min-w-64 w-[calc(100%/3)]">
      {modes.map(({ id, icon, alt, enabled }, idx) => (
        <button
          key={id}
          onClick={() => setMode(id)}
          className={`p-2 ${mode !== id ? "bg-gray-700 rounded-md" : ""}`}
          aria-label={`Switch to ${alt} mode`}
          disabled={ !enabled }
        >
          <img src={icon} alt={alt} className="h-6 w-6" />
        </button>
      ))}
    </div>
  );
}

export default NavigationTabs;
