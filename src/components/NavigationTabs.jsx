import React from "react";
import recorderIcon from "../assets/recorder.svg";
import editorIcon from "../assets/editor.svg";
import settingsIcon from "../assets/settings.svg";

const modes = [
  { id: "recorder", icon: recorderIcon, alt: "Recorder" },
  { id: "editor", icon: editorIcon, alt: "Editor" },
  { id: "settings", icon: settingsIcon, alt: "Settings" },
];

function NavigationTabs({ mode, setMode }) {
  return (
    <div className="flex justify-evenly bg-gray-800 p-3 rounded-lg max-w-96 min-w-64 w-[calc(100%/3)]">
      {modes.map(({ id, icon, alt }) => (
        <button
          key={id}
          onClick={() => setMode(id)}
          className={`p-2 ${mode !== id ? "bg-gray-700 rounded-md" : ""}`}
          aria-label={`Switch to ${alt} mode`}
        >
          <img src={icon} alt={alt} className="h-6 w-6" />
        </button>
      ))}
    </div>
  );
}

export default NavigationTabs;
