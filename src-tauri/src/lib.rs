// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod commands;
use std::path::PathBuf;

use commands::*;

#[allow(unused_variables)]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(not(debug_assertions))]
            let app_cache_dir = tauri::PathResolver::app_cache_dir(app).unwrap();
            #[cfg(debug_assertions)]
            let app_cache_dir = {
                let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join("target")
                    .join("app-cache");
                if !path.exists() {
                    std::fs::create_dir_all(&path).expect("failed to create app cache dir");
                }
                path
            };
            xlab_core::set_app_cache_dir(app_cache_dir);
            std::thread::spawn(move || xlab_core::init());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_recording,
            recording_state,
            stop_recording,
            save_recording,
            discard_recording,
            available_resolutions,
            available_frame_rates,
            update_resolution,
            update_pointer,
            update_frame_rate,
            saving_progress,
            past_videos,
            remove_previous_recording_by_index
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
