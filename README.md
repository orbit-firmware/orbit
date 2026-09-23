# orbit Firmware

<img src="https://github.com/orbit-firmware/orbit/blob/master/docs/public/logo.svg?raw=true" width="100" height="100">


orbit is a `rust keyboard firmware` built for ease.  
  
Its main selling points are:
  - You can configure your keyboard directly through the keyboard's flash drive.
  - It’s fast and reliable, as it's built in Rust.
  - Adding your own keyboard is as simple as creating a single configuration file.
  - It runs on chips supported by [embassy](https://github.com/embassy-rs/embassy).
    Today that is the `STM32F411CEU` (see `orbit/chips/`); other embassy targets need a chip crate.

# [Documentation](https://orbit-firmware.github.io/orbit) 👈


The docs are powered by [VitePress](https://vitepress.dev/). They are also viewable offline via `just docs`.

If you need help, we have a friendly [Discord](https://discord.gg/SrESTtBKV5) server for you.

## License

orbit is licensed under either of your choice

- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)
