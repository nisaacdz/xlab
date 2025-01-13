use std::sync::{Mutex, OnceLock};

use xcap::image::{Rgba, RgbaImage};

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
        let resolution = (1366, 768);
        Mutex::new(UserOptions::new(pointer, frame_rate, resolution))
    })
}

pub fn get_pointers() -> &'static Vec<Box<dyn Pointer + Send + Sync>> {
    POINTERS.get_or_init(move || {
        vec![
            Box::new(InvisiblePointer),
            Box::new(SystemPointer),
            Box::new(SolidPointer::new(draw_pointer_1(), (10, 10))),
            Box::new(SolidPointer::new(draw_pointer_2(), (12, 12))),
            Box::new(SolidPointer::new(draw_pointer_3(), (15, 15))),
            Box::new(SolidPointer::new(draw_pointer_4(), (15, 15))),
        ]
    })
}

pub fn update_resolution(mut width: u32, height: u32) {
    // Making sure the width is divisible by 2 (bit manipulation)
    // important for ffmpeg
    width = width & !1;
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

/// Generates a 20x20 pointer with two concentric circles.
fn draw_pointer_1() -> RgbaImage {
    let size = 21;
    let mut image = RgbaImage::new(size, size);
    let inner_radius = 2;
    let outer_radius = size as i32 / 2;

    let inner_color = Rgba([215, 85, 0, 255]); // Black, fully opaque
    let outer_color = Rgba([215, 85, 0, 90]);  // Black, translucent

    let center = size as i32 / 2;

    for i in -outer_radius..=outer_radius {
        for j in -outer_radius..=outer_radius {
            let distance_sq = i * i + j * j;

            if distance_sq <= outer_radius * outer_radius {
                let color = if distance_sq <= inner_radius * inner_radius {
                    inner_color
                } else {
                    outer_color
                };
                image.put_pixel((center + i) as u32, (center + j) as u32, color);
            }
        }
    }

    image
}

/// Generates a cross with a thickened center.
fn draw_pointer_2() -> RgbaImage {
    let size = 25;
    let mut image = RgbaImage::new(size, size);
    let thick = 3i32;
    let length = size as i32 / 2;
    let center = size as i32 / 2;
    let color = Rgba([0, 0, 0, 255]);
    let outer_color = Rgba([255, 255, 255, 120]);
    let inner_length = length - 2;
    let inner_thick = thick - 2;

    for i in -length..=length {
        for j in -thick..=thick {
            if i.abs() > inner_length || (i.abs() > thick && j.abs() > inner_thick) {
                image.put_pixel((center + i) as u32, (center + j) as u32, outer_color);
                image.put_pixel((center + j) as u32, (center + i) as u32, outer_color);
            } else {
                image.put_pixel((center + i) as u32, (center + j) as u32, color);
                image.put_pixel((center + j) as u32, (center + i) as u32, color);
            }
        }
    }

    image
}

/// Generates concentric rings with a dot in the center.
fn draw_pointer_3() -> RgbaImage {
    let size = 31;
    let mut image = RgbaImage::new(size, size);
    let radii = [5, 10, 15];
    let center = size as i32 / 2;
    let color = Rgba([0, 0, 0, 255]);

    for &radius in &radii {
        for i in -radius..=radius {
            for j in -radius..=radius {
                let dist_sq = i * i + j * j;
                if dist_sq <= radius * radius && dist_sq >= (radius - 2) * (radius - 2) {
                    image.put_pixel((center + i) as u32, (center + j) as u32, color);
                }
            }
        }
    }

    image.put_pixel(center as u32, center as u32, color);
    image
}

/// Generates a diamond cross.
fn draw_pointer_4() -> RgbaImage {
    let size = 31;
    let mut image = RgbaImage::new(size, size);
    let color = Rgba([0, 0, 0, 255]);
    let outer_color = Rgba([255, 255, 255, 128]);
    image.put_pixel(0, 0, color);
    image.put_pixel(size - 1, 0, color);
    for i in 1..size {
        image.put_pixel(i, i, color);
        image.put_pixel(i, i - 1, outer_color);
        image.put_pixel(i - 1, i, outer_color);

        image.put_pixel(i, size - i - 1, color);
        image.put_pixel(i, size - i, outer_color);
        image.put_pixel(i - 1, size - i - 1, outer_color);
    }
    image.put_pixel(size - 1, size - 1, color);
    image.put_pixel(0, size - 1, color);

    image
}

#[test]
fn save_pointers() {
    let pointers = [
        ("pointer_1.png", draw_pointer_1()),
        ("pointer_2.png", draw_pointer_2()),
        ("pointer_3.png", draw_pointer_3()),
        ("pointer_4.png", draw_pointer_4()),
    ];

    for (filename, img) in pointers.iter() {
        img.save(filename).unwrap();
    }
}
