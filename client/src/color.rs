#[cfg(feature = "benchmark")]
use std::time::Instant;

use crate::command_parsing::Algorithm;
use std::{collections::HashMap, ops::Add};
use x11::xlib::{
    Display, Window, XAllPlanes, XDestroyImage, XGetImage, XGetPixel, XImage, XWindowAttributes,
    ZPixmap,
};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

fn simple_average(screenshot: *mut XImage, width: u32, height: u32) -> Color {
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

    Color { r, g, b }
}

fn most_used(screenshot: *mut XImage, width: u32, height: u32) -> Color {
    // The value will be how many pixels of the color there are on the screen
    let mut colors: HashMap<Color, u32> = HashMap::new();

    // i looooove nested loops!!
    for x in 0..width {
        for y in 0..height {
            let pixel = unsafe { XGetPixel(screenshot, x as i32, y as i32) };
            let color = Color {
                r: ((pixel >> 16) & 0xFF) as u8,
                g: ((pixel >> 8) & 0xFF) as u8,
                b: (pixel & 0xFF) as u8,
            };

            match colors.get(&color) {
                Some(existing_value) => {
                    colors.insert(color, existing_value.add(1));
                }

                None => {
                    colors.insert(color, 0);
                }
            }
        }
    }

    let mut max_value: u32 = 0;
    let mut most_used_color: Option<Color> = None;

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
        Color { r: 0, g: 0, b: 0 }
    }
}

pub fn get_average_color(
    display: *mut Display,
    window: *mut Window,
    attr: XWindowAttributes,
    algorithm: &Algorithm,
) -> Color {
    let width = attr.width as u32;
    let height = attr.height as u32;

    let screenshot =
        unsafe { XGetImage(display, *window, 0, 0, width, height, XAllPlanes(), ZPixmap) };

    #[cfg(feature = "benchmark")]
    let start_time = Instant::now();

    let averaged_color = match algorithm {
        Algorithm::SimpleAverage => simple_average(screenshot, width, height),
        Algorithm::MostUsed => most_used(screenshot, width, height),
    };

    #[cfg(feature = "benchmark")]
    println!("Elapsed time: {:?}", Instant::now() - start_time);

    unsafe { XDestroyImage(screenshot) };

    return averaged_color;
}
