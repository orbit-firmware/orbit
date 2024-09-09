#![no_main]
#![no_std]

pub mod config;
pub mod keycode;
pub mod modifiers;

pub use config::Config;
pub use keycode::KeyCode;

// pub async fn run(_config: config::Config) -> ! {
pub async fn run() -> ! {
  // let config_data = env!("CONFIG_DATA");

  loop {
    // Your processing logic here
  }
}
