// Wrapper for Tauri invoke that uses mock in development
let tauriInvoke;

try {
  // Try to import the real Tauri API
  const tauriApi = await import("@tauri-apps/api/core");
  tauriInvoke = tauriApi.invoke;
} catch (error) {
  console.log("Tauri API not available, using mock");
  tauriInvoke = null;
}

export const invoke = async (command, args) => {
  // Use mock if Tauri is not available or if mock is explicitly set
  if (!tauriInvoke && window.__TAURI_INVOKE__) {
    return window.__TAURI_INVOKE__(command, args);
  }
  
  if (tauriInvoke) {
    return tauriInvoke(command, args);
  }
  
  throw new Error("Neither Tauri API nor mock is available");
};
