use crate::config;
use crate::time;

#[derive(Copy, Clone)]
pub struct Key {
  pub index: usize,
  pub state: bool,
  pub press_time: u32,
  pub change_time: u32,
  pub debouncing: bool,
}

impl Key {
  pub fn new() -> Key {
    Key {
      index: 0,
      state: false,
      press_time: 0,
      change_time: 0,
      debouncing: false,
    }
  }

  pub fn set_index(&mut self, index: usize) {
    self.index = index;
  }

  pub fn is_pressed(&self) -> bool {
    self.state
  }

  pub fn is_released(&self) -> bool {
    !self.state
  }

  pub fn get_held_time(&self) -> u32 {
    time::elapsed(self.press_time)
  }

  fn press(&mut self) {
    self.state = true;
    self.press_time = time::now();
  }

  fn release(&mut self) {
    self.state = false;
    self.press_time = 0;
  }

  #[allow(unused)]
  fn read_state(&self) -> bool {
    let x = config::LAYOUT[self.index][0];
    let y = config::LAYOUT[self.index][1];
    false
  }

  pub fn update(&mut self) {
    let state = self.read_state();
    let now = time::now();

    if self.state != state && !self.debouncing {
      if state {
        self.press();
      } else {
        self.release();
      }
      self.change_time = now;
      self.debouncing = true;
    }

    if self.debouncing {
      if self.state != state {
        self.change_time = now;
      }
      if time::elapsed(self.change_time) > config::DEBOUNCE_MS {
        self.debouncing = false;
        if self.state != state {
          self.state = state;
        }
      }
    }
  }
}

pub struct Keyboard {
  pub key_count: usize,
  pub keys: [Key; config::KEY_COUNT],
}

impl Keyboard {
  pub fn new() -> Self {
    assert!(config::KEY_COUNT > 0);
    let mut keys = [Key::new(); config::KEY_COUNT];
    for i in 0..config::KEY_COUNT {
      keys[i].set_index(i);
    }

    Keyboard {
      key_count: config::KEY_COUNT,
      keys,
    }
  }

  pub async fn scan(&mut self) {
    for key in self.keys.iter_mut() {
      key.update();
    }
  }

  pub async fn send(&self) {
    // Send the current state of the keyboard
  }
}
