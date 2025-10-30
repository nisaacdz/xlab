import React, { useState, useEffect, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { ScrollArea } from "@/components/ui/scroll-area";
import { MoreHorizontal } from "lucide-react";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { formatDuration, formatDate, getFilename } from "../utils/formatters";
import { RecordingState, SavingState } from "../utils/api";

function formatTime(milliseconds) {
  const seconds = Math.floor(milliseconds / 1000);
  const minutes = Math.floor(seconds / 60);
  const remainingSeconds = seconds % 60;
  return `${minutes.toString().padStart(2, "0")}:${remainingSeconds
    .toString()
    .padStart(2, "0")}`;
}

export function MainPage() {
  const [recordingState, setRecordingState] = useState(null);
  const [pastVideos, setPastVideos] = useState([]);

  const pollRecordingState = useCallback(async () => {
    try {
      const state = await invoke("recording_state");
      setRecordingState(state);
    } catch (error) {
      console.error("Error polling recording state:", error);
    }
  }, []);

  const pollSavingState = useCallback(async () => {
    try {
      const state = await invoke("saving_progress");
      if (state === SavingState.DONE) {
        refreshPastVideos();
        setRecordingState({ state: RecordingState.IDLE });
      } else {
        setRecordingState({ state: RecordingState.SAVING, progress: state });
      }
    } catch (error) {
      console.error("Error polling saving state:", error);
    }
  }, []);

  const refreshPastVideos = useCallback(async () => {
    try {
      const videos = await invoke("past_videos");
      setPastVideos(videos);
    } catch (error) {
      console.error("Error fetching past videos:", error);
    }
  }, []);

  useEffect(() => {
    refreshPastVideos();
    pollRecordingState();
  }, [refreshPastVideos, pollRecordingState]);

  useEffect(() => {
    let interval;
    if (recordingState?.state === RecordingState.RECORDING) {
      interval = setInterval(() => {
        setRecordingState((prevState) => ({
          ...prevState,
          instant: prevState.instant,
        }));
      }, 1000);
    } else if (recordingState?.state === RecordingState.SAVING) {
      interval = setInterval(pollSavingState, 500);
    }
    return () => clearInterval(interval);
  }, [recordingState, pollSavingState]);

  const handleStartRecording = async () => {
    await invoke("start_recording");
    pollRecordingState();
  };

  const handleStopRecording = async () => {
    await invoke("stop_recording");
    pollRecordingState();
  };

  const handleSaveRecording = async () => {
    await invoke("save_recording");
    pollSavingState();
  };

  const handleDiscardRecording = async () => {
    await invoke("discard_recording");
    pollRecordingState();
  };

  const handleOpenLocation = async (filePath) => {
    await invoke("open_file_location", { path: filePath });
  };

  const handleRemoveRecording = async (index) => {
    await invoke("remove_previous_recording_by_index", { index });
    refreshPastVideos();
  };

  const renderRecordButton = () => {
    if (!recordingState) return null;

    switch (recordingState.state) {
      case RecordingState.IDLE:
        return (
          <Button
            onClick={handleStartRecording}
            className="w-48 h-48 rounded-full bg-red-500 hover:bg-red-600 text-white text-2xl font-bold"
          >
            Record
          </Button>
        );
      case RecordingState.RECORDING:
        return (
          <Button
            onClick={handleStopRecording}
            className="w-48 h-48 rounded-full bg-gray-500 hover:bg-gray-600 text-white text-2xl font-bold"
          >
            {formatTime(Date.now() - new Date(recordingState.instant))}
          </Button>
        );
      case RecordingState.DONE:
        return (
          <div className="flex flex-col items-center gap-4">
            <p className="text-lg">Recording finished</p>
            <div className="flex gap-4">
              <Button onClick={handleSaveRecording}>Save</Button>
              <Button onClick={handleDiscardRecording} variant="destructive">
                Discard
              </Button>
            </div>
          </div>
        );
      case RecordingState.SAVING:
        return (
          <div className="flex flex-col items-center gap-4">
            <p className="text-lg">Saving...</p>
            {/* You can add a progress bar here if you want */}
          </div>
        );
      default:
        return null;
    }
  };

  return (
    <div className="min-h-screen bg-background text-foreground p-8 flex flex-col items-center">
      <div className="w-full max-w-4xl flex flex-col items-center gap-8">
        <div className="flex items-center justify-center h-64">
          {renderRecordButton()}
        </div>

        <Card className="w-full">
          <CardHeader>
            <CardTitle>Previous Recordings</CardTitle>
          </CardHeader>
          <CardContent>
            <ScrollArea className="h-[400px]">
              <Table>
                <TableHeader>
                  <TableRow>
                    <TableHead>Filename</TableHead>
                    <TableHead>Duration</TableHead>
                    <TableHead>Date</TableHead>
                    <TableHead className="text-right">Actions</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {pastVideos.map((video, index) => (
                    <TableRow key={index}>
                      <TableCell>{getFilename(video.file_path)}</TableCell>
                      <TableCell>{formatDuration(video.duration)}</TableCell>
                      <TableCell>{formatDate(video.time_recorded)}</TableCell>
                      <TableCell className="text-right">
                        <DropdownMenu>
                          <DropdownMenuTrigger asChild>
                            <Button variant="ghost" className="h-8 w-8 p-0">
                              <span className="sr-only">Open menu</span>
                              <MoreHorizontal className="h-4 w-4" />
                            </Button>
                          </DropdownMenuTrigger>
                          <DropdownMenuContent align="end">
                            <DropdownMenuItem
                              onClick={() =>
                                handleOpenLocation(video.file_path)
                              }
                            >
                              Open location
                            </DropdownMenuItem>
                            <DropdownMenuItem
                              onClick={() => handleRemoveRecording(index)}
                              className="text-red-500"
                            >
                              Delete
                            </DropdownMenuItem>
                          </DropdownMenuContent>
                        </DropdownMenu>
                      </TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            </ScrollArea>
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
