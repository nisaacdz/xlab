import React from "react";

const CircleWithRing = () => {
  return (
    <svg xmlns="http://www.w3.org/2000/svg" width="21" height="21" viewBox="0 0 21 21">
      {/* Outer ring */}
      <circle cx="10.5" cy="10.5" r="10.5" fill="rgba(215, 85, 0, 0.35)" />
      {/* Inner core */}
      <circle cx="10.5" cy="10.5" r="3.5" fill="rgb(215, 85, 0)" />
    </svg>
  );
};

const CrossWithPadding = () => (
  <svg xmlns="http://www.w3.org/2000/svg" width="360" height="360" viewBox="0 0 360 360">
    {/* Padding around the cross */}
    <rect x="0" y="158" width="360" height="44" fill="rgb(255, 255, 255)" />
    <rect x="158" y="0" width="44" height="360" fill="rgb(255, 255, 255)" />

    {/* Inner black cross (core) */}
    <rect x="11" y="169" width="338" height="22" fill="rgb(0, 0, 0)" />
    <rect x="169" y="11" width="22" height="338" fill="rgb(0, 0, 0)" />

    {/* Center square */}
    <rect x="158" y="158" width="44" height="44" fill="rgb(0, 0, 0)" />
  </svg>
);

const ConcentricCircles = () => (
  <svg width="360" height="360" xmlns="http://www.w3.org/2000/svg">
    {/* Outer Circle */}
    <circle cx="180" cy="180" r="168" stroke="black" strokeWidth="24" fill="none" />
    
    {/* Middle Circle */}
    <circle cx="180" cy="180" r="108" stroke="black" strokeWidth="24" fill="none" />
    
    {/* Inner Circle */}
    <circle cx="180" cy="180" r="48" stroke="black" strokeWidth="24" fill="none" />
  </svg>
);

const DiagonalCross = () => (
  <svg width="360" height="360" xmlns="http://www.w3.org/2000/svg">
    {/* Define the padding around the diagonal lines */}
    <line x1="360" y1="0" x2="0" y2="360" stroke="white" strokeWidth="44" />
    <line x1="0" y1="0" x2="360" y2="360" stroke="white" strokeWidth="44" />

    {/* Define the diagonal line from top-left to bottom-right */}
    <line x1="11" y1="11" x2="349" y2="349" stroke="black" strokeWidth="22" />
    
    {/* Define the diagonal line from top-right to bottom-left */}
    <line x1="349" y1="11" x2="11" y2="349" stroke="black" strokeWidth="22" />
  </svg>
);

const pointers = [CircleWithRing, CrossWithPadding, ConcentricCircles, DiagonalCross];

export default function renderSolidPointer(value) {
  return pointers[(value - 2) % pointers.length]();
}