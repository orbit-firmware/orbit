#[cfg(not(feature = "mock_enabled"))]
use embassy_time::Instant;

#[cfg(feature = "mock_enabled")]
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(not(feature = "mock_enabled"))]
pub fn now() -> u32 {
  Instant::now().as_millis() as u32
}

#[cfg(feature = "mock_enabled")]
pub fn now() -> u32 {
  let start = SystemTime::now();
  let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
  since_the_epoch.as_secs() as u32
}

pub fn elapsed(time: u32) -> u32 {
  now() - time
}
