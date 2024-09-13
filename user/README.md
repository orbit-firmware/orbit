# Example RMK Keyboard setup

📖 [Docs](https://rmk-firmware.github.io/rmk)

The important files are:
 - `keyboard.toml` - keyboard configuration
 - `custom.rs` - can be used to extend keyboard functionality (optional)
 - `keymap.rmk` - Your own keymap (not used for compile process)

 These files will be placed in the `user` directory of the firmware.  
 If present they will be available in the compile process.

 RMK also works with github actions, an example configuration can be found [here](https://github.com/rmk-firmware/example/blob/master/.github/workflows/build.yml).

 The action will provide you with the compiled firmware for your keyboard.
 Once flashed, you can drag your `keymap.rmk` onto the flash drive of the keyboard.