use std::{
    path::PathBuf,
    sync::Mutex,
    time::{Duration, Instant},
};
use xcap::image::RgbaImage;

#[derive(Clone, Copy)]
pub enum RecordingState {
    Idle,
    Recording(Instant),
    Done(Duration),
}

pub struct RecordOptions {
    pub(crate) pointer: &'static (dyn Pointer + Send + Sync),
    pub(crate) frame_rate: u32,
    pub(crate) resolution: (u32, u32),
    pub cache_count: Mutex<u64>,
    recording_state: Mutex<RecordingState>,
    pub session_name: String,
    pub output_dir: PathBuf,
    pub cache_dir: PathBuf,
}

impl RecordOptions {
    pub fn new(
        pointer: &'static (dyn Pointer + Send + Sync),
        frame_rate: u32,
        resolution: (u32, u32),
        session_name: String,
        output_dir: PathBuf,
        cache_dir: PathBuf,
    ) -> Self {
        Self {
            pointer,
            frame_rate,
            resolution,
            cache_count: Mutex::new(0),
            recording_state: Mutex::new(RecordingState::Idle),
            session_name,
            output_dir,
            cache_dir,
        }
    }

    pub fn start_recording(&self) {
        *self.recording_state.lock().unwrap() = RecordingState::Recording(Instant::now())
    }

    pub fn is_recording(&self) -> bool {
        matches!(
            *self.recording_state.lock().unwrap(),
            RecordingState::Recording(_)
        )
    }

    pub fn end_recording(&self) -> Option<Duration> {
        let mut recording_state = self.recording_state.lock().unwrap();
        match *recording_state {
            RecordingState::Idle => None,
            RecordingState::Recording(instant) => {
                let duration = instant.elapsed();
                *recording_state = RecordingState::Done(duration);
                Some(instant.elapsed())
            }
            RecordingState::Done(duration) => Some(duration),
        }
    }

    pub fn get_rate(&self) -> u32 {
        self.frame_rate
    }

    pub fn get_resolution(&self) -> (u32, u32) {
        self.resolution
    }

    pub fn cache_count(&self) -> u64 {
        *self.cache_count.lock().unwrap()
    }

    pub fn next_cache_count(&self) -> u64 {
        let mut cache_count = self.cache_count.lock().unwrap();
        *cache_count += 1;
        *cache_count
    }

    pub fn get_pointer(&self) -> &'static (dyn Pointer + Send + Sync) {
        self.pointer
    }

    pub fn cache_dir(&self) -> &PathBuf {
        &self.cache_dir
    }

    pub fn output_dir(&self) -> &PathBuf {
        &self.output_dir
    }

    pub fn recording_duration(&self) -> Option<Duration> {
        match *self.recording_state.lock().unwrap() {
            RecordingState::Idle => None,
            RecordingState::Recording(instant) => Some(instant.elapsed()),
            RecordingState::Done(duration) => Some(duration),
        }
    }
}

pub trait Pointer {
    fn resolve(&self, screen: &mut RgbaImage, position: (u32, u32));
}

pub struct InvisiblePointer;

impl Pointer for InvisiblePointer {
    fn resolve(&self, _screen: &mut RgbaImage, _position: (u32, u32)) {
        // Do nothing
    }
}

pub struct SolidPointer {
    image: RgbaImage,
    hotspot: (u32, u32),
}

impl SolidPointer {
    pub fn new(image: RgbaImage, hotspot: (u32, u32)) -> Self {
        Self { image, hotspot }
    }
}

impl Pointer for SolidPointer {
    fn resolve(&self, screen: &mut RgbaImage, position: (u32, u32)) {
        let (pointer_width, pointer_height) = (self.image.width(), self.image.height());
        let (screen_width, screen_height) = (screen.width(), screen.height());
        let (hotspot_x, hotspot_y) = self.hotspot;

        for x in 0..pointer_width {
            for y in 0..pointer_height {
                let (i, j) = (
                    position.0 as i32 + x as i32 - hotspot_x as i32,
                    position.1 as i32 + y as i32 - hotspot_y as i32,
                );

                if i >= 0 && i < screen_width as i32 && j >= 0 && j < screen_height as i32 {
                    let screen_pixel = screen.get_pixel_mut(i as u32, j as u32);
                    let cursor_pixel = self.image.get_pixel(x, y);
                    let depth = cursor_pixel[3] as u32;
                    (0..4).for_each(|i| {
                        screen_pixel[i] = ((cursor_pixel[i] as u32 * depth
                            + screen_pixel[i] as u32 * (255 - depth))
                            / 255) as u8;
                    });
                }
            }
        }
    }
}

pub struct SystemPointer;

impl Pointer for SystemPointer {
    fn resolve(&self, screen: &mut RgbaImage, position: (u32, u32)) {
        let pointer_image = {
            // get the current pointer appearance as rgba image
            RgbaImage::new(16, 16)
        };
        let (pointer_width, pointer_height) = (pointer_image.width(), pointer_image.height());
        let (screen_width, screen_height) = (screen.width(), screen.height());
        let hotspot = (8, 8); // Assume default system hotspot is center

        for x in 0..pointer_width {
            for y in 0..pointer_height {
                let (i, j) = (
                    position.0 as i32 + x as i32 - hotspot.0 as i32,
                    position.1 as i32 + y as i32 - hotspot.1 as i32,
                );

                if i >= 0 && i < screen_width as i32 && j >= 0 && j < screen_height as i32 {
                    let screen_pixel = screen.get_pixel_mut(i as u32, j as u32);
                    let cursor_pixel = pointer_image.get_pixel(x, y);
                    let depth = cursor_pixel[3] as u32;
                    (0..4).for_each(|i| {
                        screen_pixel[i] = ((cursor_pixel[i] as u32 * depth
                            + screen_pixel[i] as u32 * (255 - depth))
                            / 255) as u8;
                    });
                }
            }
        }
    }
}
