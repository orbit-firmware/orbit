use crate::chip::Chip;

pub struct Config {
  pub chip: Chip,
}

impl Config {
  pub fn new() -> Self {
    Config {
      chip: Chip::UNKNOWN,
    }
  }
}
