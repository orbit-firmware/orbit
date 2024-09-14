use std::io;
use device_query::{DeviceQuery, DeviceState, Keycode};
use crossterm::{
  cursor,
  terminal::{self, enable_raw_mode, disable_raw_mode, ClearType},
  ExecutableCommand,
};

use crate::orbit::{
  keyboard::Keyboard,
  log::dump,
};

pub async fn emulate() -> ! {
  let mut stdout = io::stdout();

  stdout.execute(terminal::Clear(ClearType::All)).unwrap();
  stdout.execute(cursor::MoveTo(0, 0)).unwrap();
  stdout.execute(cursor::Hide).unwrap();
  enable_raw_mode().unwrap();
  let mut keyboard = Keyboard::new();
  dump!("Emulating keyboard input...");
  dump!("Press 'Ctrl + C' to quit");
  dump!("Now listening for keypresses...");
  let device_state = DeviceState::new();    
  
  loop {
    let keys: Vec<Keycode> = device_state.get_keys();
    if keys.len() > 0 {
      let lctrl = keys.contains(&Keycode::LControl);
      let rctrl = keys.contains(&Keycode::RControl);
      if (lctrl || rctrl) && keys.contains(&Keycode::C) {
        dump!("Exiting...");
        disable_raw_mode().unwrap();
        stdout.execute(cursor::Show).unwrap();
        std::process::exit(0);
      }
    }
      
    keyboard.process().await;
  }
}
