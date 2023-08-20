use color::types::RGB;
use gumdrop::Options;
use hidapi::HidApi;
use std::time::Duration;
use undici::x11::display::Display;

mod color;
mod command_parsing;
mod hid;

fn main() {
    let args = command_parsing::Arguments::parse_args_default_or_exit();

    let display = Display::new().expect("can't open monitor");
    let root_window = display.get_root_window();

    let api = HidApi::new().unwrap();

    let device = api
        .device_list()
        .find(|device| hid::is_my_device(device))
        .unwrap()
        .open_device(&api)
        .unwrap();

    let mut previous_color: Option<RGB> = None;

    loop {
        let average_color = color::get_average_color(&root_window, &args.algorithm);

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
