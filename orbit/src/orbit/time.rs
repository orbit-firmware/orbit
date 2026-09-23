#[cfg(not(feature = "chip_type_emulator"))]
use embassy_time::Instant;

// milliseconds since start; the emulator counts on a monotonic clock, so a wall clock
// change cannot move it
pub fn now() -> u32 {
  #[cfg(feature = "chip_type_emulator")]
  {
    static START: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();
    return START.get_or_init(std::time::Instant::now).elapsed().as_millis() as u32;
  }
  #[cfg(not(feature = "chip_type_emulator"))]
  return Instant::now().as_millis() as u32;
}
