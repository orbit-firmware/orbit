mod orbit {
  #[cfg(not(feature = "chip_type_emulator"))]
  pub mod boot;
  pub mod config;
  pub mod dbg;
  pub mod engine;
  #[cfg(not(feature = "chip_type_emulator"))]
  pub mod hid;
  pub mod key;
  pub mod keyboard;
  pub mod keycodes;
  pub mod keymap;
  pub mod peripherals;
  pub mod process;
  pub mod time;
}
