use std::time::SystemTime;

use tauri_plugin_dialog::DialogExt;
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
    let save_at_chosen_loc = |save_fn: Box<dyn FnOnce(Option<std::path::PathBuf>) + Send>| {
        let temp_filename = format! {"rec_{}_xlab.mp4", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()};
        let dialog = DialogExt::dialog(super::APP_HANDLE.get().unwrap());
        dialog
            .file()
            .set_file_name(&temp_filename)
            .add_filter("MP4 Files", &["mp4"])
            .save_file(move |filepath| save_fn(filepath.map(|v| v.into_path().ok()).flatten()));
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
pub fn get_current_resolution() -> [u32; 2] {
    let options = xlab_core::user::get_user_options();
    let options = options.lock().unwrap();
    let (width, height) = options.resolution;
    [width, height]
}

#[tauri::command]
pub fn get_current_frame_rate() -> u32 {
    let options = xlab_core::user::get_user_options();
    let options = options.lock().unwrap();
    options.frame_rate
}

#[tauri::command]
pub fn get_current_pointer() -> usize {
    let options = xlab_core::user::get_user_options();
    let options = options.lock().unwrap();
    let pointers = xlab_core::user::get_pointers();
    // Find the index of the current pointer
    pointers.iter().position(|p| std::ptr::eq(p.as_ref() as *const _, options.pointer as *const _)).unwrap_or(0)
}

#[tauri::command]
pub fn capture_current_pointer_image() -> Option<Vec<u8>> {
    // Capture the current system pointer and return it as PNG bytes
    // This can be used for compositing into videos
    // Returns None if capture is not available or fails
    None // TODO: Implement actual pointer capture
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

#[tauri::command]
pub fn open_file_location(path: String) -> Result<(), String> {
    use std::process::Command;
    
    let path = std::path::Path::new(&path);
    
    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .args(["/select,", path.to_str().unwrap()])
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    
    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    
    Ok(())
}
