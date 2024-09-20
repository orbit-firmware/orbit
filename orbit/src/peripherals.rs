use device_query::{DeviceQuery, DeviceState, Keycode};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Peripheral {
  None,
  Q,
  W,
  E,
  R,
}

pub struct Peripherals {
  device_state: DeviceState,
}

pub struct InputPin<'a> {
  device_state: &'a DeviceState,
  key: Keycode,
}

impl<'a> InputPin<'a> {
  pub fn new(device_state: &'a DeviceState, key: Keycode) -> Self {
    Self { device_state, key }
  }

  pub fn is_high(&self) -> bool {
    let keys = self.device_state.get_keys();
    keys.contains(&self.key)
  }

  pub fn is_low(&self) -> bool {
    let keys = self.device_state.get_keys();
    !keys.contains(&self.key)
  }
}

pub struct OutputPin {}

impl OutputPin {
  pub fn new() -> Self {
    Self {}
  }

  pub fn set_high(&self) {}

  pub fn set_low(&self) {}
}

impl Peripherals {
  pub fn new() -> Self {
    Self {
      device_state: DeviceState::new(),
    }
  }

  pub fn input(&self, p: Peripheral) -> InputPin {
    match p {
      Peripheral::Q => InputPin::new(&self.device_state, Keycode::Q),
      Peripheral::W => InputPin::new(&self.device_state, Keycode::W),
      Peripheral::E => InputPin::new(&self.device_state, Keycode::E),
      Peripheral::R => InputPin::new(&self.device_state, Keycode::R),
      _ => panic!("Invalid input peripheral"),
    }
  }

  pub fn output(&self, p: Peripheral) -> OutputPin {
    match p {
      _ => panic!("Invalid output peripheral"),
    }
  }
}
