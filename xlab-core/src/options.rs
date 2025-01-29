use std::{
    path::PathBuf,
    sync::Mutex,
    time::{Duration, Instant, SystemTime},
};
use xcap::image::RgbaImage;

#[derive(Clone, Copy, serde::Serialize)]
pub enum RecordingState {
    Idle,
    #[serde(serialize_with = "serialize_instant")]
    Recording(Instant),
    #[serde(serialize_with = "serialize_duration")]
    Done(Duration),
}

fn serialize_instant<S>(instant: &Instant, sz: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let r_time = SystemTime::now()
        .checked_sub(instant.elapsed())
        .unwrap_or(SystemTime::now());
    sz.serialize_u128(
        r_time
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis(),
    )
}

fn serialize_duration<S>(duration: &Duration, sz: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    sz.serialize_u64(duration.as_millis() as u64)
}

#[test]
fn test_recording_state_serialization() {
    let instant = Instant::now();
    let time = SystemTime::now();

    let recording_state = RecordingState::Recording(instant);
    std::thread::sleep(Duration::from_secs(3));

    use serde_json::to_string;

    let recording_state_str = to_string(&recording_state).unwrap();

    // Simulate a JavaScript Object
    #[allow(non_snake_case)]
    #[derive(serde::Deserialize)]
    struct RecordingState2 {
        Recording: u128,
    }

    let recording_state2: RecordingState2 = serde_json::from_str(&recording_state_str).unwrap();
    let time_discrepancy = recording_state2.Recording
        - time
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis();
    println!("Time discrepancy: {time_discrepancy}");
}

impl RecordingState {
    pub fn duration(&self) -> Duration {
        match self {
            RecordingState::Idle => Duration::from_secs(0),
            RecordingState::Recording(instant) => instant.elapsed(),
            RecordingState::Done(duration) => *duration,
        }
    }
}

pub struct RecordOptions {
    pub(crate) pointer: &'static (dyn Pointer + Send + Sync),
    pub(crate) frame_rate: u32,
    pub(crate) resolution: (u32, u32),
    pub cache_count: Mutex<u64>,
    pub(crate) recording_state: Mutex<RecordingState>,
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

    pub fn is_done_recording(&self) -> bool {
        matches!(
            *self.recording_state.lock().unwrap(),
            RecordingState::Done(_)
        )
    }

    pub fn end_recording(&self) -> Option<Duration> {
        let mut recording_state = self.recording_state.lock().unwrap();
        let record_duration = recording_state.duration();
        *recording_state = RecordingState::Done(record_duration);
        Some(record_duration)
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

    pub fn recording_state(&self) -> RecordingState {
        *self.recording_state.lock().unwrap()
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
        draw_image_on_screen(screen, position, &self.image, self.hotspot);
    }
}

pub struct SystemPointer;

impl Pointer for SystemPointer {
    fn resolve(&self, screen: &mut RgbaImage, position: (u32, u32)) {
        let pointer_image = {
            // get the current pointer appearance as rgba image
            RgbaImage::new(16, 16)
        };
        draw_image_on_screen(screen, position, &pointer_image, (0, 0));
    }
}

pub fn draw_image_on_screen(
    screen: &mut RgbaImage,
    coordinates: (u32, u32),
    image: &RgbaImage,
    image_hotspot: (u32, u32),
) {
    let (image_width, image_height) = (image.width(), image.height());
    let (screen_width, screen_height) = (screen.width(), screen.height());
    let (hotspot_x, hotspot_y) = image_hotspot;

    // Hotspot must appear at the coordinates on the screen
    // There should be a linear transformation that transforms every pixel coordinate
    // of the image to new coordinates on the screen such that the image_hotspot gets
    // translated to the coordinates location.

    for x in 0..image_width {
        for y in 0..image_height {
            let (i, j) = (
                coordinates.0 as i32 + x as i32 - hotspot_x as i32,
                coordinates.1 as i32 + y as i32 - hotspot_y as i32,
            );

            if i >= 0 && i < screen_width as i32 && j >= 0 && j < screen_height as i32 {
                let screen_pixel = screen.get_pixel_mut(i as u32, j as u32);
                let cursor_pixel = image.get_pixel(x, y);
                let depth = cursor_pixel[3] as u32;
                (0..3).for_each(|c| {
                    screen_pixel[c] = ((cursor_pixel[c] as u32 * depth
                        + screen_pixel[c] as u32 * (255 - depth))
                        / 255) as u8;
                });
                screen_pixel[3] = (((255 * cursor_pixel[3] as u32)
                    + (255 * screen_pixel[3] as u32)
                    - (cursor_pixel[3] as u32 * screen_pixel[3] as u32))
                    / 255) as u8;
            }
        }
    }
}
