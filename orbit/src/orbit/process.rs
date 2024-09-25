//
// default mode
//
#[cfg(not(feature = "chip_type_emulator"))]
mod stm32 {
  use crate::orbit::keyboard::Keyboard;
  use embassy_usb::driver::Driver;

  use crate::orbit::dbg::*;
  use crate::orbit::hid::Hid;
  use crate::orbit::peripherals::*;
  use embassy_futures::join::join;
  use embassy_time::Timer;

  pub async fn run<D: Driver<'static>>(driver: D) {
    let mut keyboard = Keyboard::instance();
    keyboard.create_hid(driver);
    keyboard.process().await;
    // let process = async {
    //   loop {
    //     if !hid.usb_ready() {
    //       Timer::after_millis(100).await;
    //       info!("Waiting for USB to be ready");
    //       continue;
    //     }

    //   }
    // };

    // process.await;
  }
}

#[cfg(not(feature = "chip_type_emulator"))]
pub use stm32::run;

//
// emulator mode
//
#[cfg(feature = "chip_type_emulator")]
mod emulator {
  use crossterm::cursor;
  use crossterm::terminal;
  use crossterm::terminal::disable_raw_mode;
  use crossterm::terminal::enable_raw_mode;
  use crossterm::terminal::ClearType;
  use crossterm::ExecutableCommand;
  use device_query::DeviceQuery;
  use device_query::DeviceState;
  use device_query::Keycode;

  use crate::orbit::dbg::dump;
  use crate::orbit::keyboard::Keyboard;

  const CLEAR: bool = false;

  pub async fn run() -> ! {
    let mut stdout = std::io::stdout();

    if CLEAR {
      stdout.execute(terminal::Clear(ClearType::All)).unwrap();
      stdout.execute(cursor::MoveTo(0, 0)).unwrap();
    }

    stdout.execute(cursor::Hide).unwrap();
    enable_raw_mode().unwrap();
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

      Keyboard::instance().process().await;
    }
  }
}

#[cfg(feature = "chip_type_emulator")]
pub use emulator::run;
