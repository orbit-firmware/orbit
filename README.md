# Orbit Firmware

<img src="https://github.com/orbit-firmware/orbit/blob/master/docs/public/logo.svg?raw=true" width="100" height="100">


Orbit is a `rust keyboard firmware` built for ease.  
  
Its main selling points are:
  - You can configure your keyboard directly through the keyboard's flash drive.
  - It’s fast and reliable, as it's built in Rust.
  - Adding your own keyboard is as simple as creating a single configuration file.
  - It supports a wide variety of chipsets through [embassy](https://github.com/embassy-rs/embassy).
    - All of the `STM32` family
    - The `nRF52`, `nRF53` and `nRF91`
    - Raspberry Pi `RP2040`
    - The `ESP32` and the WCH 32-bit RISC-V(CH32V) series.

# Documentation
[Documentation](https://orbit-firmware.github.io/orbit).

The docs are powered by VitePress. They are also viewable offline via `make docs`.

If you need help, we have a friendly [Discord](https://discord.gg/SrESTtBKV5) server for you.

## License

Orbit is licensed under either of your choice

- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)
