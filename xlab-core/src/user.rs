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
    let size = 361;
    let mut image = RgbaImage::new(size, size);
    let inner_radius = 36;
    let outer_radius = size as i32 / 2;

    let inner_color = Rgba([215, 85, 0, 255]); // Black, fully opaque
    let outer_color = Rgba([215, 85, 0, 90]); // Black, translucent

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

    super::resize_image(&mut image, (32, 32));

    image
}

/// Generates a cross with a thickened center.
fn draw_pointer_2() -> RgbaImage {
    let size = 361;
    let mut image = RgbaImage::new(size, size);

    let padding = size as i32 / 32;
    let thick = size as i32 / 8;
    let length = size as i32 / 2; // Scale length
    let center = size as i32 / 2;

    let color = Rgba([0, 0, 0, 255]); // Core color
    let outer_color = Rgba([255, 255, 255, 120]); // Outer color

    let inner_length = length - 2 * (size as i32 / 25);
    let inner_thick = thick - (2 * padding);

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

    super::resize_image(&mut image, (21, 21));

    image
}

/// Generates concentric rings with a dot in the center.
fn draw_pointer_3() -> RgbaImage {
    let size = 361;
    let thickness = 24;
    let mut image = RgbaImage::new(size, size);
    let radii = [size as i32 / 6, size as i32 / 3, size as i32 / 2];
    let center = size as i32 / 2;
    let color = Rgba([0, 0, 0, 255]);

    for &radius in &radii {
        for i in -radius..=radius {
            for j in -radius..=radius {
                let dist_sq = i * i + j * j;
                if dist_sq <= radius * radius
                    && dist_sq >= (radius - thickness) * (radius - thickness)
                {
                    image.put_pixel((center + i) as u32, (center + j) as u32, color);
                }
            }
        }
    }

    super::resize_image(&mut image, (36, 36));

    image
}

fn draw_pointer_4() -> RgbaImage {
    let size = 361;
    let thickness = 22;
    let padding = 44;
    let mut image = RgbaImage::new(size, size);
    let color = Rgba([0, 0, 0, 255]); // Core color
    let padding_color = Rgba([255, 255, 255, 255]); // Padding color

    // Draw the padding lines
    for i in 0..size {
        for j in 0..size {
            if (i as i32 - j as i32).abs() < padding
                || (i as i32 + j as i32 - size as i32).abs() < padding
            {
                image.put_pixel(i, j, padding_color);
            }
        }
    }

    // Draw the diagonal lines
    for i in 11..(size - 11) {
        for j in 11..(size - 11) {
            if (i as i32 - j as i32).abs() < thickness
                || (i as i32 + j as i32 - size as i32).abs() < thickness
            {
                image.put_pixel(i as u32, j as u32, color);
            }
        }
    }

    super::resize_image(&mut image, (21, 21));

    image
}
