use crate::orbit::key::Key;
use crate::orbit::time;
use heapless::String;

pub struct Event {
  mapping: String<32>,
  processed: bool,
  index: usize,
  state: bool,
  taps: u16,
  timestamp: u64,
  // enabled_behaviors: {
  //   Press: bool,
  //   Hold: bool,
  //   Tap,: bool,
  //   Modding: bool,
  // }
}

impl Event {
  pub fn from_key(key: Key) -> Event {
    let mut mapping = String::new();
    mapping.push_str("a").unwrap();

    Self {
      mapping,
      processed: false,
      state: key.state(),
      index: key.index(),
      timestamp: key.timestamp(),
      taps: key.taps(),
    }
  }

  pub fn mapping(&self) -> &str {
    self.mapping.as_str()
  }

  pub fn set_mapping(&mut self, mapping: &str) {
    self.mapping.clear();
    let _ = self.mapping.push_str(mapping);
  }

  pub fn processed(&self) -> bool {
    self.processed
  }

  pub fn send(&mut self) {
    self.processed = true;
  }

  pub fn index(&self) -> usize {
    self.index
  }

  pub fn time(&self) -> u64 {
    time::elapsed(self.timestamp)
  }

  pub fn state(&self) -> bool {
    self.state
  }

  pub fn taps(&self) -> u16 {
    self.taps
  }
}
