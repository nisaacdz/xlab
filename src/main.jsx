import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";
import App from "./App.jsx";
import { setupMockTauri } from "./utils/mockTauri.js";

// Setup mock Tauri API for development without backend
setupMockTauri();

createRoot(document.getElementById("root")).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
