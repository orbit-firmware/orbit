#[cfg(not(feature = "emulator_enabled"))]
use embassy_time::Instant;

#[cfg(feature = "emulator_enabled")]
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(not(feature = "emulator_enabled"))]
pub fn now() -> u64 {
  Instant::now().as_millis() as u64
}

#[cfg(feature = "emulator_enabled")]
pub fn now() -> u64 {
  let start = SystemTime::now();
  let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
  since_the_epoch.as_millis() as u64
}

pub fn elapsed(time: u64) -> u64 {
  now() - time
}
