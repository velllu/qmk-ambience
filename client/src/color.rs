use x11::xlib::{
    Display, Window, XAllPlanes, XDestroyImage, XGetImage, XGetPixel, XWindowAttributes, ZPixmap,
};

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub fn get_average_color(
    display: *mut Display,
    window: *mut Window,
    attr: XWindowAttributes,
) -> Color {
    let width = attr.width as u32;
    let height = attr.height as u32;

    let screenshot =
        unsafe { XGetImage(display, *window, 0, 0, width, height, XAllPlanes(), ZPixmap) };

    let (mut r, mut g, mut b) = (0u64, 0u64, 0u64);

    for x in 0..width {
        for y in 0..height {
            let pixel = unsafe { XGetPixel(screenshot, x as i32, y as i32) };
            r += (pixel >> 16) & 0xFF;
            g += (pixel >> 8) & 0xFF;
            b += pixel & 0xFF;
        }
    }

    unsafe { XDestroyImage(screenshot) };

    let total_pixels: u64 = width as u64 * height as u64;
    let r: u8 = (r / total_pixels) as u8;
    let g: u8 = (g / total_pixels) as u8;
    let b: u8 = (b / total_pixels) as u8;

    Color { r, g, b }
}
