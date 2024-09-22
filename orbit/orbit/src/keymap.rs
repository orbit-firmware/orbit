use crate::orbit::config;
use core::option::Option;
use heapless::String;

const MAX_LAYERS: usize = 16;

pub struct KeyMap {
  data: [Option<String<32>>; config::KEY_COUNT * config::BEHAVIOR_COUNT * MAX_LAYERS],
}

impl KeyMap {
  pub fn load_data(&mut self) {
    // load in from flash storage
  }

  pub fn set(&mut self, key: u8, layer: u8, behavior: u8, token: &str) {
    let index = self.get_index(key as usize, layer as usize, behavior as usize);
    let mut t = String::new();
    t.push_str(token).unwrap();
    self.data[index] = Some(t);
  }

  pub fn get(&self, key: u8, layer: u8, behavior: u8) -> &str {
    let index = self.get_index(key as usize, layer as usize, behavior as usize);

    if let Some(ref token) = self.data[index] {
      return token.as_str();
    }

    ""
  }

  fn get_index(&self, key: usize, layer: usize, behavior: usize) -> usize {
    key * config::BEHAVIOR_COUNT * MAX_LAYERS + behavior * MAX_LAYERS + layer
  }
}
