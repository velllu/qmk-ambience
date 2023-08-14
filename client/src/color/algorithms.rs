#[cfg(feature = "benchmark")]
use std::time::Instant;

use super::types::RGB;
use crate::color::types::HSV;
use std::{collections::HashMap, ops::Add};
use x11::xlib::{XGetPixel, XImage};

/// 1. Adds all screen RGB values into three counters
/// 2. Divide them by the pixel count
/// 3. Boom, you get a color that looks like ass most of the time but is very fast to
/// compute
pub fn simple_average(screenshot: *mut XImage, width: u32, height: u32) -> RGB {
    let (mut r, mut g, mut b) = (0u64, 0u64, 0u64);

    for x in 0..width {
        for y in 0..height {
            let pixel = unsafe { XGetPixel(screenshot, x as i32, y as i32) };
            r += (pixel >> 16) & 0xFF;
            g += (pixel >> 8) & 0xFF;
            b += pixel & 0xFF;
        }
    }

    let total_pixels: u64 = width as u64 * height as u64;
    let r: u8 = (r / total_pixels) as u8;
    let g: u8 = (g / total_pixels) as u8;
    let b: u8 = (b / total_pixels) as u8;

    RGB { r, g, b }
}

/// 1. Count how much each Color is used, make brighter colors be counted as more
/// 2. Get the most used one
pub fn most_used(screenshot: *mut XImage, width: u32, height: u32) -> RGB {
    // The value will be how many pixels of the color there are on the screen
    let mut colors: HashMap<RGB, f32> = HashMap::new();

    // i looooove nested loops!!
    for x in 0..width {
        for y in 0..height {
            let pixel = unsafe { XGetPixel(screenshot, x as i32, y as i32) };
            let color = RGB {
                r: ((pixel >> 16) & 0xFF) as u8,
                g: ((pixel >> 8) & 0xFF) as u8,
                b: (pixel & 0xFF) as u8,
            };

            match colors.get(&color) {
                Some(existing_value) => {
                    let color_hsv: HSV = color.into();
                    colors.insert(color, existing_value.add(color_hsv.s * color_hsv.v));
                }

                None => {
                    colors.insert(color, 0.);
                }
            }
        }
    }

    let mut max_value: f32 = 0.;
    let mut most_used_color: Option<RGB> = None;

    for (color, number_of_pixels) in colors.iter() {
        if *number_of_pixels > max_value {
            max_value = *number_of_pixels;

            // TODO: this could be optimized to not be copied every time. I don't think
            // it's actually a problem because the `Color` array is *very* small
            most_used_color = Some(*color);
        }
    }

    #[cfg(debug_assertions)]
    println!("Most used pixel color: {}", max_value);

    if most_used_color.is_some() {
        most_used_color.unwrap()
    } else {
        RGB { r: 0, g: 0, b: 0 }
    }
}
