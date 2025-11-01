use std::{path::PathBuf, sync::OnceLock, time::SystemTime};

use serde::Deserialize;
use user::get_pointers;

static APP_CACHE_DIR: OnceLock<PathBuf> = OnceLock::new();
use fast_image_resize::images::Image;
use fast_image_resize::{PixelType, Resizer};
use xcap::image::RgbaImage;

/// This function when called first before app starts, initializes several static variables
/// and prevents initializations during runtime
/// When called after app has started, it does nothing
pub fn init() {
    let _ = get_pointers();
}

pub(crate) fn resize_image(img: &mut RgbaImage, (new_width, new_height): (u32, u32)) {
    // Convert RgbaImage to fast_image_resize::images::Image
    let (old_width, old_height) = img.dimensions();
    let buffer_mut = unsafe {
        std::slice::from_raw_parts_mut(
            img.as_mut_ptr(),
            old_width as usize * old_height as usize * 4,
        )
    };
    let old_img = Image::from_slice_u8(old_width, old_height, buffer_mut, PixelType::U8x4).unwrap();

    // Create a new image with the desired dimensions
    let mut new_img = Image::new(new_width, new_height, PixelType::U8x4);

    // Resize the image
    Resizer::new()
        .resize(&old_img, &mut new_img, None)
        .expect("Failed to resize image");

    // Replace the original image with the resized image
    *img = RgbaImage::from_vec(new_width, new_height, new_img.into_vec()).unwrap();
}

pub fn set_app_cache_dir(app_cache_dir: PathBuf) {
    if !app_cache_dir.exists() {
        std::fs::create_dir_all(&app_cache_dir).expect("failed to create app cache dir");
    }
    APP_CACHE_DIR.set(app_cache_dir).ok();
}

pub fn get_app_cache_dir() -> Option<&'static PathBuf> {
    APP_CACHE_DIR.get()
}

pub fn get_app_cache_output_dir() -> PathBuf {
    get_app_cache_dir().unwrap().join("recordings")
}

pub fn screen_resolution() -> (u32, u32) {
    let monitor = xcap::Monitor::all().unwrap().into_iter().next().unwrap();
    (monitor.width().unwrap(), monitor.height().unwrap())
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

fn log_new_recording(file_path: PathBuf, duration: u64) {
    let mut recordings = previous_recordings();

    if let Some(index) = recordings.iter().position(|v| &v.file_path == &file_path) {
        recordings.remove(index);
    }

    let recording = PreviousRecording {
        time_recorded: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        duration,
        file_path,
        resolution: screen_resolution(),
    };

    recordings.push(recording);
    let serialized = serde_json::to_string(&recordings).unwrap();
    std::fs::write(completed_recordings_log(), serialized).unwrap();
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PreviousRecording {
    #[serde(
        serialize_with = "serialize_time_recorded",
        deserialize_with = "deserialize_time_recorded"
    )]
    time_recorded: u64, // time as duration since unix epoch
    duration: u64,
    #[serde(
        serialize_with = "serialize_path_buf",
        deserialize_with = "deserialize_path_buf"
    )]
    file_path: PathBuf,
    resolution: (u32, u32),
}

fn serialize_path_buf<S>(path_buf: &PathBuf, sz: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    sz.serialize_str(path_buf.to_str().unwrap())
}

fn deserialize_path_buf<'de, D>(dz: D) -> Result<PathBuf, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let path_str = String::deserialize(dz)?;
    Ok(PathBuf::from(path_str))
}

fn serialize_time_recorded<S>(time: &u64, sz: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::Serialize;
    #[derive(Serialize)]
    struct TimeRecorded {
        secs_since_epoch: u64,
    }
    TimeRecorded { secs_since_epoch: *time }.serialize(sz)
}

fn deserialize_time_recorded<'de, D>(dz: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum TimeRecorded {
        Object { secs_since_epoch: u64 },
        Number(u64),
    }
    match TimeRecorded::deserialize(dz)? {
        TimeRecorded::Object { secs_since_epoch } => Ok(secs_since_epoch),
        TimeRecorded::Number(v) => Ok(v),
    }
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
    use user::{update_frame_rate, update_pointer, update_resolution};

    use super::*;

    #[test]
    fn record_screen() {
        let app_cache_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("app-cache");
        set_app_cache_dir(app_cache_dir);
        update_pointer(2);
        update_frame_rate(24);
        let width = 1366 * 720 / 768;
        update_resolution(width, 720);
        record();
        std::thread::sleep(Duration::from_secs(12));
        stop();
        save_video(|save_fn| save_fn(None));
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
