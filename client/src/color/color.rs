#[cfg(feature = "benchmark")]
use std::time::Instant;

use undici::x11::window::Window;

use super::types::RGB;
use crate::{
    color::{
        algorithms::{most_used, simple_average},
        types::HSV,
    },
    command_parsing::Algorithm,
};

pub fn get_average_color(root_window: &Window, algorithm: &Algorithm) -> RGB {
    let screenshot = root_window.get_image();
    let width = screenshot.width;
    let height = screenshot.height;

    #[cfg(feature = "benchmark")]
    let start_time = Instant::now();

    let averaged_color = match algorithm {
        Algorithm::SimpleAverage => simple_average(screenshot, width, height),
        Algorithm::MostUsed => most_used(screenshot, width, height),
    };

    #[cfg(feature = "benchmark")]
    println!("Elapsed time: {:?}", Instant::now() - start_time);

    #[cfg(debug_assertions)]
    print!(
        "Choosen color: \x1b[48;2;{};{};{}m\x1b[30m             \x1b[0m\n",
        averaged_color.r, averaged_color.g, averaged_color.b
    );

    #[cfg(debug_assertions)]
    {
        let hsv: HSV = averaged_color.into();
        println!(
            "R: {:x}, G: {:x}, B: {:x}.    H: {}, S: {}, V: {}",
            averaged_color.r, averaged_color.g, averaged_color.b, hsv.h, hsv.s, hsv.v
        );
    }

    return averaged_color;
}
