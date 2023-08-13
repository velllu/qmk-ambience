use super::types::RGB;
use crate::{
    color::algorithms::{most_used, simple_average},
    command_parsing::Algorithm,
};
use x11::xlib::{
    Display, Window, XAllPlanes, XDestroyImage, XGetImage, XWindowAttributes, ZPixmap,
};

pub fn get_average_color(
    display: *mut Display,
    window: *mut Window,
    attr: XWindowAttributes,
    algorithm: &Algorithm,
) -> RGB {
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

    #[cfg(debug_assertions)]
    print!(
        "Choosen color: \x1b[48;2;{};{};{}m\x1b[30m             \x1b[0m\n",
        averaged_color.r, averaged_color.g, averaged_color.b
    );

    unsafe { XDestroyImage(screenshot) };

    return averaged_color;
}
