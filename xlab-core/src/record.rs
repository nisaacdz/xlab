use std::{
    path::PathBuf,
    sync::{Mutex, OnceLock},
    time::Duration,
};

use mouse_position::mouse_position;
use xcap::{
    image::{ImageBuffer, Rgba},
    Monitor,
};

use super::options::{Pointer, RecordOptions};

static MONITOR: OnceLock<Monitor> = OnceLock::new();
static OPTIONS: OnceLock<Mutex<RecordOptions>> = OnceLock::new();
static RECORD_HANDLE: OnceLock<Mutex<Option<std::thread::JoinHandle<()>>>> = OnceLock::new();
static SAVE_HANDLE: OnceLock<Mutex<Option<std::thread::JoinHandle<()>>>> = OnceLock::new();
static SAVE_PROGRESS: OnceLock<Mutex<Option<SaveProgress>>> = OnceLock::new();

pub type RgbaImage = ImageBuffer<Rgba<u8>, Vec<u8>>;

fn get_monitor() -> &'static Monitor {
    MONITOR.get_or_init(move || xcap::Monitor::all().unwrap().into_iter().next().unwrap())
}

fn get_options() -> &'static Mutex<RecordOptions> {
    OPTIONS.get_or_init(move || {
        let user_options = super::user::get_user_options().lock().unwrap();
        let frame_rate = user_options.frame_rate;
        let resolution = user_options.resolution;
        let pointer = user_options.pointer;
        let cache_dir = super::CACHE_DIR.get().unwrap().clone();
        let data_dir = super::DATA_DIR.get().unwrap().clone();
        Mutex::new(RecordOptions::new(
            pointer, frame_rate, resolution, cache_dir, data_dir,
        ))
    })
}

pub(crate) fn get_record_handle() -> &'static Mutex<Option<std::thread::JoinHandle<()>>> {
    RECORD_HANDLE.get_or_init(|| Mutex::new(None))
}

pub(crate) fn get_save_handle() -> &'static Mutex<Option<std::thread::JoinHandle<()>>> {
    SAVE_HANDLE.get_or_init(|| Mutex::new(None))
}

pub fn get_save_progress() -> &'static Mutex<Option<SaveProgress>> {
    SAVE_PROGRESS.get_or_init(|| Mutex::new(None))
}

pub fn record() {
    let handle = std::thread::spawn(move || {
        let record_options = get_options();
        let mut record_options_lock = record_options.lock().unwrap();
        const ONE_MICRO: u64 = 1000_000;
        let frame_rate = record_options_lock.get_rate();
        let pointer = record_options_lock.get_pointer();
        let cache_dir = record_options_lock.get_cache_dir().clone();
        let session_name = generate_random_string(8);
        record_options_lock.session_name = session_name.clone();
        std::mem::drop(record_options_lock);
        let mut background_tasks = Vec::new();
        // create the cache_dir if it does not exist and clear it if it does
        if cache_dir.exists() {
            std::fs::remove_dir_all(&cache_dir).unwrap();
        }
        std::fs::create_dir_all(&cache_dir).unwrap();
        let wait_duration = Duration::from_micros(ONE_MICRO / frame_rate as u64);

        while !record_options.lock().unwrap().session_ended() {
            let start = std::time::Instant::now();
            let cache_count = record_options.lock().unwrap().next_cache_count();
            let image_dir = generate_cached_image_path(&cache_dir, &session_name, cache_count);
            let monitor = get_monitor();
            let screen: RgbaImage = monitor.capture_image().unwrap();
            let pointer_position = get_mouse_position();
            background_tasks.push(std::thread::spawn(move || {
                process(image_dir, pointer, screen, pointer_position)
            }));
            std::thread::sleep(
                wait_duration
                    .checked_sub(start.elapsed())
                    .unwrap_or_default(),
            );
        }
        for task in background_tasks {
            task.join().unwrap();
        }
        // I don't think a thread should be able to join itself
        // bad code : get_record_handle().lock().unwrap().take().unwrap().join().unwrap();
    });
    let old_handle = get_record_handle().lock().unwrap().replace(handle);
    if let Some(old_handle) = old_handle {
        old_handle.join().unwrap();
    }
}

pub fn clear_cache() {}

pub fn save_video() {
    assert!(matches!(
        get_save_progress().lock().unwrap().as_ref(),
        None | Some(SaveProgress::Done)
    ));
    get_save_progress()
        .lock()
        .unwrap()
        .replace(SaveProgress::Initializing);
    let handle = std::thread::spawn(move || {
        let record_options = get_options();
        let record_options = record_options.lock().unwrap();
        assert!(record_options.session_ended());
        let cache_dir = record_options.get_cache_dir().clone();
        let data_dir = record_options.get_data_dir().clone();
        // create the data_dir if it does not exist and clear it if it does
        if data_dir.exists() {
            std::fs::remove_dir_all(&data_dir).unwrap();
        }
        std::fs::create_dir_all(&data_dir).unwrap();
        let last_idx = record_options.cache_count();
        let session_name = record_options.session_name.clone();
        let frame_rate = record_options.get_rate();
        let resolution = record_options.get_resolution();
        std::mem::drop(record_options);
        let output_path = generate_output_path(&data_dir, &session_name);
        let mut video_encoder =
            super::video::VideoEncoder::initialize(output_path, frame_rate, resolution, Default::default()).unwrap();
        for cache_count in 1..=last_idx {
            get_save_progress()
                .lock()
                .unwrap()
                .replace(SaveProgress::Saving(cache_count, last_idx));
            // png image is at image_path
            // append the image to the video
            let image_path = generate_cached_image_path(&cache_dir, &session_name, cache_count);
            video_encoder.append_image(&image_path).unwrap();
        }

        get_save_progress()
            .lock()
            .unwrap()
            .replace(SaveProgress::Finalizing);

        // save video here
        video_encoder.finalize().unwrap();

        if cache_dir.exists() {
            std::fs::remove_dir_all(cache_dir).unwrap();
        }

        get_save_progress()
            .lock()
            .unwrap()
            .replace(SaveProgress::Done);
    });
    get_save_handle().lock().unwrap().replace(handle).map(|v| v.join());
}

pub fn stop() {
    let record_options = get_options();
    let record_options = record_options.lock().unwrap();
    record_options.end_session();
    //get_record_handle().lock().unwrap().take().unwrap().join().unwrap();
}

pub fn process(
    image_path: PathBuf,
    pointer: &'static dyn Pointer,
    mut screen: RgbaImage,
    pointer_position: (u32, u32),
) {
    pointer.resolve(&mut screen, pointer_position);
    screen.save(image_path).unwrap();
}

fn get_mouse_position() -> (u32, u32) {
    match mouse_position::Mouse::get_mouse_position() {
        mouse_position::Mouse::Position { x, y } => (x as u32, y as u32),
        mouse_position::Mouse::Error => todo!(),
    }
}

fn generate_random_string(length: usize) -> String {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    let bytes = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .collect();
    String::from_utf8(bytes).unwrap()
}

fn generate_cached_image_path(
    cache_dir: &PathBuf,
    session_name: &str,
    cache_count: u32,
) -> PathBuf {
    cache_dir.join(format!("{session_name}_{:07}.png", cache_count))
}

fn generate_output_path(data_dir: &PathBuf, session_name: &str) -> PathBuf {
    data_dir.join(format!("__{session_name}__.mp4"))
}

pub enum SaveProgress {
    Initializing,
    Saving(u32, u32),
    Finalizing,
    Done,
}
