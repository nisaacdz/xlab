use std::{path::PathBuf, sync::OnceLock, time::SystemTime};

static APP_CACHE_DIR: OnceLock<PathBuf> = OnceLock::new();

pub fn set_app_cache_dir(app_cache_dir: PathBuf) {
    if !app_cache_dir.exists() {
        std::fs::create_dir_all(&app_cache_dir).expect("failed to create app cache dir");
    }
    APP_CACHE_DIR.set(app_cache_dir).ok();
}

pub fn get_app_cache_dir() -> Option<&'static PathBuf> {
    APP_CACHE_DIR.get()
}

pub fn screen_resolution() -> (u32, u32) {
    let monitor = record::get_monitor();
    (monitor.width(), monitor.height())
}

pub fn valid_resolutions() -> [[u32; 2]; 8] {
    let (width, height) = screen_resolution();
    const RAW_RESOLUTIONS: [u32; 8] = [144, 240, 360, 480, 720, 1080, 1440, 2160];
    (0..RAW_RESOLUTIONS.len())
        .map(|i| {
            let resolution = RAW_RESOLUTIONS[i];
            let aspect_ratio = width as f32 / height as f32;
            let new_width = (resolution as f32 * aspect_ratio).round() as u32;
            [new_width, resolution]
        })
        .collect::<Vec<[u32; 2]>>()
        .try_into()
        .unwrap()
}

pub fn previous_recordings() -> Vec<PreviousRecording> {
    let recordings = match std::fs::read_to_string(completed_recordings_log())
        .ok()
        .map(|v| serde_json::from_str(&v).ok())
        .flatten()
    {
        Some(u) => u,
        None => Vec::new(),
    };
    recordings
}

pub fn delete_previous_recording(index: usize) {
    let mut recordings = previous_recordings();
    recordings.remove(index);
    let serialized = serde_json::to_string(&recordings).unwrap();
    std::fs::write(completed_recordings_log(), serialized).unwrap();
}

fn log_new_recording(file_path: PathBuf, duration: std::time::Duration) {
    let duration = duration.as_secs();
    let recording = PreviousRecording {
        time_recorded: SystemTime::now(),
        duration,
        file_path,
        resolution: screen_resolution(),
    };
    let mut recordings = previous_recordings();
    recordings.push(recording);
    let serialized = serde_json::to_string(&recordings).unwrap();
    std::fs::write(completed_recordings_log(), serialized).unwrap();
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PreviousRecording {
    time_recorded: SystemTime,
    duration: u64,
    #[serde(serialize_with = "serialize_path_buf")]
    file_path: PathBuf,
    resolution: (u32, u32),
}

fn serialize_path_buf<S>(path_buf: &PathBuf, sz: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    sz.serialize_str(path_buf.to_str().unwrap())
}

fn completed_recordings_log() -> PathBuf {
    let cache_path = get_app_cache_dir().unwrap();
    let log_path = cache_path.join("prev_recordings.json");
    std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open(&log_path)
        .unwrap();
    log_path
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use record::{record, save_video, stop};
    use user::{update_frame_rate, update_pointer};

    use super::*;

    #[test]
    fn record_screen() {
        let app_cache_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("app-cache");
        set_app_cache_dir(app_cache_dir);
        update_pointer(1);
        update_frame_rate(24);
        record();
        std::thread::sleep(Duration::from_secs(25));
        stop();
        super::record::get_record_handle()
            .lock()
            .unwrap()
            .take()
            .map(|u| u.join());
        save_video();
        super::record::get_save_handle()
            .lock()
            .unwrap()
            .take()
            .map(|v| v.join());
    }
}

pub mod options;
pub mod record;
pub mod user;
pub mod video;
