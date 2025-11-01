import React from "react";

export function SettingsMode() {
  return (
    <div className="glass card" style={{ height: '100%', display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center' }}>
      <h2 className="card-header">Settings</h2>
      <p style={{ color: 'var(--text-secondary)', textAlign: 'center' }}>
        Coming soon: Configure your recording preferences
      </p>
    </div>
  );
}
