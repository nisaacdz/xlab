import React from "react";
import { CursorArrowRaysIcon, CursorArrowRippleIcon, EyeSlashIcon } from "@heroicons/react/24/solid";
import "./PointerPreview.css";

const pointerOptions = [
  { 
    id: 0, 
    name: "Hidden", 
    description: "No cursor visible",
    Icon: EyeSlashIcon,
    preview: null
  },
  { 
    id: 1, 
    name: "System", 
    description: "Default system cursor",
    Icon: CursorArrowRaysIcon,
    preview: "system"
  },
  { 
    id: 2, 
    name: "Solid", 
    description: "Highlighted solid cursor",
    Icon: CursorArrowRippleIcon,
    preview: "solid"
  }
];

export function PointerPreview({ currentPointer, onSelect, disabled }) {
  return (
    <div className="pointer-preview-container">
      <label className="pointer-label">Mouse Cursor:</label>
      <div className="pointer-options">
        {pointerOptions.map((option) => {
          const Icon = option.Icon;
          const isSelected = currentPointer === option.id;
          
          return (
            <button
              key={option.id}
              className={`pointer-option glass ${isSelected ? 'selected' : ''}`}
              onClick={() => !disabled && onSelect(option.id)}
              disabled={disabled}
              title={option.description}
            >
              <div className="pointer-icon-wrapper">
                <Icon className="pointer-icon" />
              </div>
              <span className="pointer-name">{option.name}</span>
            </button>
          );
        })}
      </div>
    </div>
  );
}
