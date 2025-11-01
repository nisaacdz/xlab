import React from "react";

export function EditorMode() {
  return (
    <div className="glass card" style={{ height: '100%', display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center' }}>
      <h2 className="card-header">Video Editor</h2>
      <p style={{ color: 'var(--text-secondary)', textAlign: 'center' }}>
        Coming soon: Edit your recordings with advanced tools
      </p>
    </div>
  );
}
