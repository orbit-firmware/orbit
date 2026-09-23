mod orbit {
  pub mod config;
  pub mod dbg;
  pub mod features;
  #[cfg(not(feature = "chip_type_emulator"))]
  pub mod hid;
  pub mod key;
  pub mod keyboard;
  pub mod keycodes;
  pub mod keymap;
  pub mod modifiers;
  pub mod peripherals;
  pub mod process;
  pub mod report;
  pub mod time;
}
