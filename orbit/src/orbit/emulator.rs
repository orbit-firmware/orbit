use std::io;
use device_query::DeviceQuery;
use device_query::DeviceState;
use device_query::Keycode;
use crossterm::cursor;
use crossterm::terminal;
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::ClearType;
use crossterm::ExecutableCommand;
use crate::orbit::keyboard::Keyboard;
use crate::orbit::log::dump;

const CLEAR : bool = true;

pub async fn emulate() -> ! {
  let mut stdout = io::stdout();

  if CLEAR {
    stdout.execute(terminal::Clear(ClearType::All)).unwrap();
    stdout.execute(cursor::MoveTo(0, 0)).unwrap();
  }
  
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
        disable_raw_mode().unwrap();
        stdout.execute(cursor::Show).unwrap();
        std::process::exit(0);
      }
    }
      
    keyboard.process().await;
  }
}
