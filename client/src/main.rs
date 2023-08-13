use gumdrop::Options;
use hidapi::HidApi;
use std::{
    ptr::{null, null_mut},
    time::Duration,
};
use x11::xlib::{XDefaultRootWindow, XGetWindowAttributes, XOpenDisplay, XWindowAttributes};

mod color;
mod command_parsing;
mod hid;

fn new_window_attributes() -> XWindowAttributes {
    // what c libraries do to a mf
    XWindowAttributes {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        border_width: 0,
        depth: 0,
        visual: null_mut(),
        root: 0,
        class: 0,
        bit_gravity: 0,
        win_gravity: 0,
        backing_store: 0,
        backing_planes: 0,
        backing_pixel: 0,
        save_under: 0,
        colormap: 0,
        map_installed: 0,
        map_state: 0,
        all_event_masks: 0,
        your_event_mask: 0,
        do_not_propagate_mask: 0,
        override_redirect: 0,
        screen: null_mut(),
    }
}

fn main() {
    let args = command_parsing::Arguments::parse_args_default_or_exit();

    let display = unsafe { XOpenDisplay(null()) };
    if display.is_null() {
        panic!("Can't open monitor");
    }

    let api = HidApi::new().unwrap();

    let device = api
        .device_list()
        .find(|device| hid::is_my_device(device))
        .unwrap()
        .open_device(&api)
        .unwrap();

    let mut previous_color: Option<color::Color> = None;

    loop {
        let mut root = unsafe { XDefaultRootWindow(display) };
        let mut attributes: XWindowAttributes = new_window_attributes();
        unsafe { XGetWindowAttributes(display, root, &mut attributes) };

        let average_color =
            color::get_average_color(display, &mut root, attributes, &args.algorithm);

        if previous_color.is_some() {
            if average_color != previous_color.unwrap() {
                #[cfg(debug_assertions)]
                println!(
                    "Sent color: #{:x}{:x}{:x}",
                    average_color.r, average_color.g, average_color.b
                );

                let _ = device.write(&[0x00, average_color.r, average_color.g, average_color.b]);
            }
        }

        previous_color = Some(average_color);

        std::thread::sleep(Duration::from_millis(args.ms));
    }
}
