import React from "react";
import { TrashIcon, FolderOpenIcon, PlayIcon } from "@heroicons/react/24/outline";
import { invoke } from "@tauri-apps/api/core";
import "./PastRecordings.css";

const formatDuration = (seconds) => {
  const hours = Math.floor(seconds / 3600);
  const mins = Math.floor((seconds % 3600) / 60);
  const secs = seconds % 60;

  if (hours > 0) {
    return `${String(hours).padStart(2, "0")}:${String(mins).padStart(2, "0")}:${String(secs).padStart(2, "0")}`;
  }
  if (mins > 0) {
    return `${String(mins).padStart(2, "0")}:${String(secs).padStart(2, "0")}`;
  }
  return `00:${String(secs).padStart(2, "0")}`;
};

const formatDate = (secsSinceEpoch) => {
  // Handle both direct number and object with secs_since_epoch property
  const timestamp = typeof secsSinceEpoch === 'object' && secsSinceEpoch.secs_since_epoch 
    ? secsSinceEpoch.secs_since_epoch 
    : secsSinceEpoch;
    
  const date = new Date(timestamp * 1000);
  const now = new Date();
  const diffMs = now - date;
  const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24));
  
  if (diffDays === 0) {
    return "Today";
  } else if (diffDays === 1) {
    return "Yesterday";
  } else if (diffDays < 7) {
    return `${diffDays} days ago`;
  } else {
    return date.toLocaleDateString();
  }
};

const getFilename = (filePath) => filePath.split("/").pop().split("\\").pop();

export function PastRecordings({ pastVideos, removeRecording }) {
  const handleCardClick = async (videoPath) => {
    try {
      await invoke("open_file_location", { path: videoPath });
    } catch (error) {
      console.error("Error opening video location:", error);
    }
  };

  const handleDelete = (e, index) => {
    e.stopPropagation(); // Prevent card click
    if (window.confirm("Are you sure you want to delete this recording?")) {
      removeRecording(index);
    }
  };

  const handleOpenLocation = async (e, videoPath) => {
    e.stopPropagation(); // Prevent card click
    try {
      await invoke("open_file_location", { path: videoPath });
    } catch (error) {
      console.error("Error opening file location:", error);
    }
  };

  return (
    <div className="past-recordings">
      <h2 className="card-header">Your Recordings</h2>
      
      {!pastVideos || pastVideos.length === 0 ? (
        <div className="empty-state">
          <PlayIcon className="empty-icon" />
          <p className="empty-text">No recordings yet</p>
          <p className="empty-subtext">Start recording to see your videos here</p>
        </div>
      ) : (
        <div className="recordings-grid">
          {pastVideos.map((video, index) => (
            <div
              key={index}
              className="recording-card glass"
              onClick={() => handleCardClick(video.file_path)}
            >
              <div className="recording-preview">
                <PlayIcon className="play-icon" />
              </div>
              
              <div className="recording-info">
                <h3 className="recording-title">
                  {getFilename(video.file_path)}
                </h3>
                
                <div className="recording-meta">
                  <span className="recording-duration">
                    {formatDuration(video.duration)}
                  </span>
                  <span className="recording-date">
                    {formatDate(video.time_recorded.secs_since_epoch)}
                  </span>
                </div>
              </div>
              
              <div className="recording-actions">
                <button
                  className="action-button"
                  onClick={(e) => handleOpenLocation(e, video.file_path)}
                  title="Open file location"
                >
                  <FolderOpenIcon className="action-icon" />
                </button>
                <button
                  className="action-button delete"
                  onClick={(e) => handleDelete(e, index)}
                  title="Delete recording"
                >
                  <TrashIcon className="action-icon" />
                </button>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
