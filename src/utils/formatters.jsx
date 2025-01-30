export const formatDuration = (totalSeconds) => {
  const hours = Math.floor(totalSeconds / 3600);
  const mins = Math.floor((totalSeconds % 3600) / 60);
  const secs = totalSeconds % 60;

  console.log("totalSeconds", totalSeconds);

  if (hours > 0) {
    return `${String(hours).padStart(2, "0")}:${String(mins).padStart(2, "0")}:${String(secs).padStart(2, "0")}`;
  }
  if (mins > 0) {
    return `${String(mins).padStart(2, "0")}:${String(secs).padStart(2, "0")}`;
  }
  return `00:${String(secs).padStart(2, "0")}`;
};

export const formatTime = (time) => {
  const ms = String(Math.floor((time % 1000) / 10)).padStart(2, "0");
  const seconds = String(Math.floor((time / 1000) % 60)).padStart(2, "0");
  const minutes = String(Math.floor((time / (1000 * 60)) % 60)).padStart(
    2,
    "0",
  );
  const hours = String(Math.floor(time / (1000 * 60 * 60))).padStart(2, "0");

  return (
    <div className="font-mono flex items-center space-x-1">
      <span className="text-2xl">{hours}</span>
      <span className="text-gray-400">:</span>
      <span className="text-2xl">{minutes}</span>
      <span className="text-gray-400">:</span>
      <span className="text-2xl">{seconds}</span>
      <span className="text-gray-400">.</span>
      <span className="text-xl text-gray-800">{ms}</span>
    </div>
  );
};

export const formatDate = (secsSinceEpoch) => {
  const date = new Date(secsSinceEpoch * 1000);
  return date.toLocaleString();
};

export const getFilename = (filePath) => filePath.split("/").pop();
