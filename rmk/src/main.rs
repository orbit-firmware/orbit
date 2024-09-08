#![no_main]
#![no_std]

pub mod chip;
pub mod config;
pub mod keycode;
pub mod modifiers;

pub use chip::Chip;
pub use config::Config;
pub use keycode::KeyCode;

#[cfg(feature = "chip_stm32f303cb")]
mod chip_stm32f303cb;

pub async fn run(_config: config::Config) -> ! {
  #[cfg(feature = "stm32f303cb")]
  let chip = Chip::STM32F303CB;

  loop {
    // Your processing logic here
  }
}
