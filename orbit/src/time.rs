#[cfg(not(feature = "family_EMULATOR"))]
use embassy_time::Instant;

#[cfg(feature = "family_EMULATOR")]
mod emulation_time {
  use core::option::Option;
  use std::time::{Duration, SystemTime, UNIX_EPOCH};

  static mut START_TIME: Option<Duration> = None;

  pub fn now() -> u32 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");

    unsafe {
      if START_TIME.is_none() {
        START_TIME = Some(since_the_epoch);
      }

      let start_time = START_TIME.unwrap();
      (since_the_epoch - start_time).as_millis() as u32
    }
  }
}

pub fn now() -> u32 {
  #[cfg(feature = "family_EMULATOR")]
  return emulation_time::now() as u32;
  #[cfg(not(feature = "family_EMULATOR"))]
  return Instant::now().as_millis() as u32;
}

pub fn elapsed(time: u32) -> u16 {
  (now() - time) as u16
}
