use std::{path::PathBuf, sync::Mutex};

use xcap::image::RgbaImage;

pub struct RecordOptions {
    pointer: &'static (dyn Pointer + Send + Sync),
    frame_rate: u32,
    resolution: (u32, u32),
    cache_dir: PathBuf,
    data_dir: PathBuf,
    cache_count: Mutex<u32>,
    session_ended: Mutex<bool>,
    pub session_name: String,
}

impl RecordOptions {
    pub fn new(
        pointer: &'static (dyn Pointer + Send + Sync),
        frame_rate: u32,
        resolution: (u32, u32),
        cache_dir: PathBuf,
        data_dir: PathBuf,
    ) -> Self {
        Self {
            pointer,
            frame_rate,
            resolution,
            cache_dir,
            data_dir,
            cache_count: Mutex::new(0),
            session_ended: Mutex::new(false),
            session_name: String::new(),
        }
    }

    pub fn get_rate(&self) -> u32 {
        self.frame_rate
    }

    pub fn get_resolution(&self) -> (u32, u32) {
        self.resolution
    }

    pub fn cache_count(&self) -> u32 {
        *self.cache_count.lock().unwrap()   
    }

    pub fn next_cache_count(&self) -> u32 {
        let mut cache_count = self.cache_count.lock().unwrap();
        *cache_count += 1;
        *cache_count
    }

    pub fn get_cache_dir(&self) -> &PathBuf {
        &self.cache_dir
    }

    pub fn get_data_dir(&self) -> &PathBuf {
        &self.data_dir
    }

    pub fn get_pointer(&self) -> &'static (dyn Pointer + Send + Sync) {
        self.pointer
    }

    pub fn end_session(&self) {
        let mut session_ended = self.session_ended.lock().unwrap();
        *session_ended = true;
    }

    pub fn session_ended(&self) -> bool {
        *self.session_ended.lock().unwrap()
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
    pub fn new(image: RgbaImage) -> Self {
        Self { image, hotspot: (0, 0) }
    }
}

impl Pointer for SolidPointer {
    fn resolve(&self, screen: &mut RgbaImage, position: (u32, u32)) {
        let (pointer_width, pointer_height) = (self.image.width(), self.image.height());
        let (screen_width, screen_height) = (screen.width(), screen.height());
        for x in 0..pointer_width {
            for y in 0..pointer_height {
                let (i, j) = (x + position.0, y + position.1);
                if i < screen_width && j < screen_height {
                    let screen_pixel = screen.get_pixel_mut(i, j);
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
        for x in 0..pointer_width {
            for y in 0..pointer_height {
                let (i, j) = (x as i32 + position.0 as i32, y as i32 + position.1 as i32);
                if i >= 0 && i < screen_width as i32 && j >= 0 && j < screen_height as i32 {
                    let screen_pixel = screen.get_pixel_mut(i as u32, j as u32);
                    let depth = 255;
                    let cursor_pixel = pointer_image.get_pixel(x, y);
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