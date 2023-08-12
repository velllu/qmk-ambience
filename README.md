# QMK Ambience
Make your keyboard follow the display colors.

# TODOs
- [-] Add a proper color picking algorithm (like pywal's) instead of just averaging the colors. This often gives a bland grey color.
    - [x] Simple averaging algorithm
    - [x] Most used algorithm (picks the most used color in the screen)
    - [ ] Still have to figure out one I am 100% satisfied with
- [ ] Add wayland support.
- [ ] Make color transitions smoother.
- [ ] Add possibility to customize the delay of sending colors to the keyboard. (As of now, to change it, you can go change the `std::thread::sleep(Duration::from_millis(500))` in `client/src/main.rs` to whatever you want)

# Installing
## Flashing firmware into the keyboard
Note that installing this **will disable VIA**!
1. Install QMK following [this](https://docs.qmk.fm/#/newbs_getting_started) guide.  
2. Run `qmk setup`.  
3. Go into the `qmk_firmware` directory, and search for your particular keyboard in the `keyboards` (e.g. `keyboards/keychron/q3/ansi`), CD into it.
4. Edit the `info.json` file to this:
```json
{
    ...
    "usb": {
        "vid": "0xFEED",
        "pid": "0x0000",
        "device_version": "1.0.0"
    }
    ...
}
```

5. Be sure that `rules.mk` has this lines, if it doesnt't add them.
```
RGB_MATRIX_ENABLE = yes # If yours doesn't have this one already enabled, your keyboard might not support this
RAW_ENABLE = yes
```

6. Edit `config.h` to the following:
```c
#pragma once

// This allow the client to search for your keyboard
#define RAW_USAGE_PAGE 0xFF60
#define RAW_USAGE_ID 0x61

...
```

7. Almost there, you probably will see a lot of folders inside `keymaps/`, you can create a new one or just edit the `default` one.

8. Open `default/keymap.c` (or your custom directory name) and modify the keyboard layout as you please.

9. Now **append** the contents of `firmware/keymap.c` (of this repo) to **your** `keymap.c`

10. Put your keyboard in bootloading mode, this changes from keyboard to keyboard, you can look [here](https://docs.qmk.fm/#/newbs_flashing), inside the "Put Your Keyboard into DFU (Bootloader) Mode" section. You will notice you are in bootloader mode if your leds don't not turn on (at least that is the case for me).

11. Run `qmk flash -kb <here put your keyboard name, e.g. "keychron/q3/ansi"> -km default`. If you named your folder, instead of "default" insert its name.

You should have now flashed the firmware. If your keyboard hopefully turns on, you may see no change, it will snap into "ambience" mode after you follow the next step

## Installing the hid client
### Dependencies
If you are a nixos user you should be able to use the shell at `client/shell.nix` fine, if you are not a nixos user, I can only point in the right direction.  
You will probably have to download (names can vary from distro to distro) `libhidapi-dev` and `libx11-dev`.

### Actually installing the client
You have three ways of installing the client
1. If you use nixos, proper packaging will arrive someday.
2. (Reccomended) If you have rust's cargo installed, you can `cd client/` and run `cargo install --path .`, you will now have `qmk-ambience-client` as a binary on your system (if you set your PATH to include rust programs).
3. (Reccomended too) You can also `cd client/` and run `cargo build --release` and running `target/release/qmk-ambience-client`.
4. You can pick the precompiled binary in the releases tab (if I remember to upload it).