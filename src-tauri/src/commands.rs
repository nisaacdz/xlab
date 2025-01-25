use std::time::SystemTime;

use xlab_core::{options::RecordingState, record::SaveProgress, PreviousRecording};

#[tauri::command]
pub fn recording_state() -> RecordingState {
    xlab_core::record::get_options()
        .lock()
        .unwrap()
        .recording_state()
}

#[tauri::command]
pub fn start_recording() {
    xlab_core::record::record();
}

#[tauri::command]
pub fn stop_recording() {
    xlab_core::record::stop();
}

#[tauri::command]
pub fn save_recording() {
    let save_at_chosen_loc = |save_fn| {
        let temp_filename = format! {"rec_{}_xlab.mp4", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()};
        tauri::api::dialog::FileDialogBuilder::new()
            .set_file_name(&temp_filename)
            .add_filter("MP4 Files", &["mp4"])
            .save_file(save_fn);
    };

    xlab_core::record::save_video(save_at_chosen_loc);
}

#[tauri::command]
pub fn discard_recording() {
    xlab_core::record::discard_video();
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
