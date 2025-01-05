use std::sync::{Mutex, OnceLock};

use super::options::{InvisiblePointer, Pointer, SolidPointer, SystemPointer};

static OPTIONS: OnceLock<Mutex<UserOptions>> = OnceLock::new();

static POINTERS: OnceLock<Vec<Box<dyn Pointer + Send + Sync>>> = OnceLock::new();

pub struct UserOptions {
    pub pointer: &'static (dyn Pointer + Send + Sync),
    pub frame_rate: u32,
    pub resolution: (u32, u32),
}

impl UserOptions {
    pub fn new(
        pointer: &'static (dyn Pointer + Send + Sync),
        frame_rate: u32,
        resolution: (u32, u32),
    ) -> Self {
        Self {
            pointer,
            frame_rate,
            resolution,
        }
    }
}

pub fn get_user_options() -> &'static Mutex<UserOptions> {
    OPTIONS.get_or_init(move || {
        let pointer = get_pointers().get(0).unwrap().as_ref();
        let frame_rate = 32;
        let resolution = (1920, 1080);
        Mutex::new(UserOptions::new(pointer, frame_rate, resolution))
    })
}

pub fn get_pointers() -> &'static Vec<Box<dyn Pointer + Send + Sync>> {
    POINTERS.get_or_init(move || {
        vec![
            Box::new(InvisiblePointer),
            Box::new(SolidPointer::new(
                xcap::image::open("/home/nisaacdz/Downloads/cursor.png")
                    .unwrap()
                    .to_rgba8(),
                    (2, 2)
            )),
            Box::new(SystemPointer),
        ]
    })
}

pub fn update_resolution(width: u32, height: u32) {
    let options = get_user_options();
    let mut options = options.lock().unwrap();
    options.resolution = (width, height);
}

pub fn update_pointer(index: usize) {
    let pointers = get_pointers();
    let pointer = pointers.get(index).unwrap().as_ref();
    let options = get_user_options();
    let mut options = options.lock().unwrap();
    options.pointer = pointer;
}

pub fn update_frame_rate(new_rate: u32) {
    let options = get_user_options();
    let mut options = options.lock().unwrap();
    options.frame_rate = new_rate;
}
