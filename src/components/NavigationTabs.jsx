import React from "react";
import { 
  VideoCameraIcon, 
  ScissorsIcon, 
  Cog6ToothIcon 
} from "@heroicons/react/24/outline";
import "./NavigationTabs.css";

const tabs = [
  { id: "recorder", Icon: VideoCameraIcon, label: "Recorder" },
  { id: "editor", Icon: ScissorsIcon, label: "Editor" },
  { id: "settings", Icon: Cog6ToothIcon, label: "Settings" },
];

function NavigationTabs({ mode, setMode }) {
  return (
    <nav className="navigation-tabs">
      <div className="tabs-container glass">
        {tabs.map(({ id, Icon, label }) => {
          const isActive = mode === id;
          
          return (
            <button
              key={id}
              onClick={() => setMode(id)}
              className={`tab-button ${isActive ? "active" : ""}`}
              aria-label={label}
              title={label}
            >
              <Icon className="tab-icon" />
              <span className="tab-label">{label}</span>
            </button>
          );
        })}
      </div>
    </nav>
  );
}

export default NavigationTabs;
