import React from "react";
import { CursorArrowRaysIcon, EyeSlashIcon } from "@heroicons/react/24/solid";
import renderSolidPointer from "./SolidPointers";
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
    name: "Circle", 
    description: "Circle with ring pointer",
    preview: "solid",
    solidIndex: 2
  },
  { 
    id: 3, 
    name: "Cross", 
    description: "Cross with padding pointer",
    preview: "solid",
    solidIndex: 3
  },
  { 
    id: 4, 
    name: "Rings", 
    description: "Concentric circles pointer",
    preview: "solid",
    solidIndex: 4
  },
  { 
    id: 5, 
    name: "X-Mark", 
    description: "Diagonal cross pointer",
    preview: "solid",
    solidIndex: 5
  }
];

export function PointerPreview({ currentPointer, onSelect, disabled }) {
  return (
    <div className="pointer-preview-container">
      <label className="pointer-label">Mouse Cursor:</label>
      <div className="pointer-options">
        {pointerOptions.map((option) => {
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
                {option.Icon ? (
                  <option.Icon className="pointer-icon" />
                ) : (
                  <div className="pointer-svg-preview">
                    {renderSolidPointer(option.solidIndex)}
                  </div>
                )}
              </div>
              <span className="pointer-name">{option.name}</span>
            </button>
          );
        })}
      </div>
    </div>
  );
}
