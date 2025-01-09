use std::time::Duration;

use xlab_core::{record::SaveProgress, PreviousRecording};

#[tauri::command]
pub fn recording_duration() -> Option<Duration> {
    xlab_core::record::get_options().lock().unwrap().end_recording()
}

#[tauri::command]
pub fn is_recording() -> bool {
    xlab_core::record::get_options().lock().unwrap().is_recording()
}

#[tauri::command]
pub fn start_recording() {
    xlab_core::record::record();
}

#[tauri::command]
pub fn stop_recording() {
    xlab_core::record::stop();
    xlab_core::record::get_record_handle()
            .lock()
            .unwrap()
            .take()
            .map(|u| u.join());
    xlab_core::record::save_video();
}

#[tauri::command]
pub fn available_resolutions() -> [[u32; 2]; 8] {
    xlab_core::valid_resolutions()
}

#[tauri::command]
pub fn available_frame_rates() -> [u32; 4] {
    [15, 24, 30, 60]
}

#[tauri::command]
pub fn update_resolution(index: usize) {
    let resolutions = xlab_core::valid_resolutions();
    let resolution = resolutions[index];
    xlab_core::user::update_resolution(resolution[0], resolution[1]);
}

#[tauri::command]
pub fn update_pointer(index: usize) {
    xlab_core::user::update_pointer(index);
}

#[tauri::command]
pub fn update_frame_rate(frame_rate: u32) {
    xlab_core::user::update_frame_rate(frame_rate);
}

#[tauri::command]
pub fn saving_progress() -> Option<SaveProgress> {
    *xlab_core::record::get_save_progress().lock().unwrap()
}

#[tauri::command]
pub fn past_videos() -> Vec<PreviousRecording> {
    xlab_core::previous_recordings()
}

#[tauri::command]
pub fn remove_previous_recording_by_index(index: usize) {
    xlab_core::delete_previous_recording(index);
}