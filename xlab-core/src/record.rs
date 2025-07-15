use std::{
    path::PathBuf,
    sync::{Mutex, OnceLock},
    time::Duration,
};

use mouse_position::mouse_position;
use xcap::image::RgbaImage;

use crate::{
    get_app_cache_dir, get_app_cache_output_dir, log_new_recording, options::RecordingState,
    user::get_user_options,
};

use super::options::{Pointer, RecordOptions};

static OPTIONS: OnceLock<Mutex<RecordOptions>> = OnceLock::new();
static RECORD_HANDLE: OnceLock<Mutex<Option<std::thread::JoinHandle<()>>>> = OnceLock::new();
static SAVE_HANDLE: OnceLock<Mutex<Option<std::thread::JoinHandle<()>>>> = OnceLock::new();
static SAVE_PROGRESS: OnceLock<Mutex<Option<SaveProgress>>> = OnceLock::new();

pub fn get_options() -> &'static Mutex<RecordOptions> {
    OPTIONS.get_or_init(move || {
        let user_options = super::user::get_user_options().lock().unwrap();
        let frame_rate = user_options.frame_rate;
        let resolution = user_options.resolution;
        let pointer = user_options.pointer;
        // these are placeholders and are guaranteed to be replaced by the record function
        let cache_dir = PathBuf::new();
        let output_dir = get_app_cache_dir().unwrap().join("recordings");
        Mutex::new(RecordOptions::new(
            pointer,
            frame_rate,
            resolution,
            String::new(),
            output_dir,
            cache_dir,
        ))
    })
}

pub fn get_record_handle() -> &'static Mutex<Option<std::thread::JoinHandle<()>>> {
    RECORD_HANDLE.get_or_init(|| Mutex::new(None))
}

pub(crate) fn get_save_handle() -> &'static Mutex<Option<std::thread::JoinHandle<()>>> {
    SAVE_HANDLE.get_or_init(|| Mutex::new(None))
}

pub fn get_save_progress() -> &'static Mutex<Option<SaveProgress>> {
    SAVE_PROGRESS.get_or_init(|| Mutex::new(None))
}

pub fn record() {
    if get_options().lock().unwrap().is_recording() {
        return;
    };
    // Starting the recording here is important for the frontend to immediately start
    // state updates after this function is called
    get_options().lock().unwrap().start_recording();
    let handle = std::thread::spawn(move || {
        let user_options_lock = get_user_options().lock().unwrap();
        let pointer = user_options_lock.pointer;
        let frame_rate = user_options_lock.frame_rate;
        let resolution = user_options_lock.resolution;
        let session_name = generate_random_string(12);
        let cache_dir = generate_session_cache_dir(&session_name);
        let output_dir = get_app_cache_output_dir();
        let new_record_options = RecordOptions::new(
            pointer,
            frame_rate,
            resolution,
            session_name.clone(),
            output_dir,
            cache_dir.clone(),
        );
        std::mem::drop(user_options_lock);
        let record_options_mtx = get_options();
        *record_options_mtx.lock().unwrap() = new_record_options;
        if cache_dir.exists() {
            std::fs::remove_dir_all(&cache_dir).ok();
        }
        std::fs::create_dir_all(&cache_dir).ok();
        const ONE_NANO: u64 = 1_000_000_000;
        let wait_duration = Duration::from_nanos(ONE_NANO / frame_rate as u64);

        // Calling start recording again will update the start time to the current time
        // Improves accuracy of the recording duration by nanoseconds (not really needed)
        // But it's good in case the above code takes a long time to execute
        let monitor = xcap::Monitor::all().unwrap().into_iter().next().unwrap();
        get_options().lock().unwrap().start_recording();

        while record_options_mtx.lock().unwrap().is_recording() {
            let start = std::time::Instant::now();
            let cache_count = record_options_mtx.lock().unwrap().next_cache_count();
            let image_dir = generate_cached_image_path(&cache_dir, &session_name, cache_count);
            let screen = monitor.capture_image().unwrap();
            let pointer_position = get_mouse_position();

            process(image_dir, pointer, screen, pointer_position);

            std::thread::sleep(
                wait_duration
                    .checked_sub(start.elapsed())
                    .unwrap_or_default(),
            );
        }
    });
    let old_handle = get_record_handle().lock().unwrap().replace(handle);
    if let Some(old_handle) = old_handle {
        old_handle.join().unwrap();
    }
}

pub fn save_video<F>(save_file_at_loc: F)
where
    F: FnOnce(Box<dyn FnOnce(Option<PathBuf>) + Send + 'static>) + Send + 'static,
{
    if !matches!(
        get_save_progress().lock().unwrap().as_ref(),
        None | Some(SaveProgress::Done)
    ) {
        return;
    }

    let options_lock = get_options().lock().unwrap();
    if !options_lock.is_done_recording() {
        return;
    };

    let recording_duration = options_lock.recording_state().duration();

    *options_lock.recording_state.lock().unwrap() = RecordingState::Idle;

    get_save_progress()
        .lock()
        .unwrap()
        .replace(SaveProgress::Initializing);

    let image_dimensions = {
        let monitor = xcap::Monitor::all().unwrap().into_iter().next().unwrap();
        (monitor.width().unwrap(), monitor.height().unwrap())
    };
    let handle = std::thread::spawn(move || {
        get_record_handle().lock().unwrap().take().map(|u| u.join());
        let record_options_mtx = get_options();
        let record_options_lock = record_options_mtx.lock().unwrap();
        let cache_dir = record_options_lock.cache_dir().clone();
        let output_dir = record_options_lock.output_dir().clone();
        let last_idx = record_options_lock.cache_count();
        let session_name = record_options_lock.session_name.clone();
        let frame_rate = record_options_lock.get_rate();
        let resolution = record_options_lock.get_resolution();
        std::mem::drop(record_options_lock);
        if !output_dir.exists() {
            std::fs::create_dir_all(&output_dir).unwrap();
        }
        let mut output_path = generate_output_path(&output_dir, &session_name);
        let mut video_encoder = super::video::VideoEncoder::new(
            output_path.clone(),
            frame_rate,
            resolution,
            image_dimensions,
            Default::default(),
        )
        .unwrap();
        for cache_count in 1..=last_idx {
            get_save_progress()
                .lock()
                .unwrap()
                .replace(SaveProgress::Saving(cache_count, last_idx));
            // png image is at image_path
            // append the image to the video
            let image_path = generate_cached_image_path(&cache_dir, &session_name, cache_count);
            let image = xcap::image::open(image_path).unwrap().to_rgba8();
            video_encoder.append_image(image, cache_count).unwrap();
        }

        get_save_progress()
            .lock()
            .unwrap()
            .replace(SaveProgress::Finalizing);

        video_encoder.finalize().unwrap();

        if cache_dir.exists() {
            std::fs::remove_dir_all(cache_dir).unwrap();
        }

        let save_fn = Box::new(move |save_path| {
            if let Some(save_path) = save_path {
                output_path = save_path;
                move_recording(&output_path);
            }
            log_new_recording(output_path, recording_duration.as_secs());

            get_save_progress()
                .lock()
                .unwrap()
                .replace(SaveProgress::Done);
        });

        save_file_at_loc(save_fn)
    });
    get_save_handle()
        .lock()
        .unwrap()
        .replace(handle)
        .map(|v| v.join());
}

pub fn discard_video() {
    let options = get_options().lock().unwrap();
    if !matches!(options.recording_state(), RecordingState::Done(_)) {
        return;
    }
    *options.recording_state.lock().unwrap() = RecordingState::Idle;
    let cache_dir = options.cache_dir().clone();
    std::thread::spawn(move || {
        if cache_dir.exists() {
            std::fs::remove_dir_all(cache_dir).ok();
        }
    });
}

pub fn stop() {
    let mut ro = get_options().lock().unwrap();
    let video_duration = ro.end_recording().unwrap();
    if video_duration.as_secs() > 2 {
        let corrected_frame_rate = ro.cache_count() / video_duration.as_secs();
        ro.frame_rate = corrected_frame_rate as u32;
    }
}

fn process(
    image_path: PathBuf,
    pointer: &'static dyn Pointer,
    mut screen: RgbaImage,
    pointer_position: (u32, u32),
) {
    pointer.resolve(&mut screen, pointer_position);
    screen.save(image_path).ok();
}

fn get_mouse_position() -> (u32, u32) {
    match mouse_position::Mouse::get_mouse_position() {
        mouse_position::Mouse::Position { x, y } => (x as u32, y as u32),
        mouse_position::Mouse::Error => (0, 0),
    }
}

pub fn generate_random_string(length: usize) -> String {
    use rand::distr::Alphanumeric;
    use rand::{rng, Rng};

    let bytes = rng().sample_iter(&Alphanumeric).take(length).collect();
    String::from_utf8(bytes).unwrap()
}

fn generate_cached_image_path(
    cache_dir: &PathBuf,
    session_name: &str,
    cache_count: u64,
) -> PathBuf {
    cache_dir.join(format!("{session_name}_{:07}.png", cache_count))
}

fn generate_session_cache_dir(session_name: &str) -> PathBuf {
    get_app_cache_dir()
        .unwrap()
        .join(format!("cache_{session_name}"))
}

fn generate_output_path(output_dir: &PathBuf, session_name: &str) -> PathBuf {
    output_dir.join(format!("__{session_name}__.mp4"))
}

pub fn move_recording(new_path: &PathBuf) {
    let session_name = get_options().lock().unwrap().session_name.clone();
    let output_path = get_options()
        .lock()
        .map(|ro| generate_output_path(ro.output_dir(), &session_name))
        .unwrap();
    if &output_path == new_path {
        return;
    };
    if !new_path.exists() {
        std::fs::create_dir_all(new_path.parent().unwrap()).unwrap();
    }
    std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open(&new_path)
        .unwrap();
    if let Err(_) = std::fs::rename(&output_path, &new_path) {
        std::fs::copy(output_path, new_path).ok();
    }
}

#[derive(serde::Serialize, Clone, Copy)]
pub enum SaveProgress {
    Initializing,
    Saving(u64, u64),
    Finalizing,
    Done,
}
