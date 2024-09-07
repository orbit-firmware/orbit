use embassy_time::Instant;

use crate::matrix::KeyState;

use super::{DebounceState, DebouncerTrait, DEBOUNCE_THRESHOLD};

/// Debounce counter info for each key.
#[derive(Copy, Clone, Debug)]
struct Debounce(u16);

impl Debounce {
  fn increase(&mut self, elapsed_ms: u16) {
    // Prevent overflow
    if u16::MAX - self.0 <= elapsed_ms {
      self.0 = u16::MAX;
    } else {
      self.0 += 1;
    }
  }

  fn decrease(&mut self, elapsed_ms: u16) {
    if elapsed_ms > self.0 {
      self.0 = 0;
    } else {
      self.0 -= 1;
    }
  }
}

/// Default per-key debouncer. The debouncing algorithm is same as ZMK's [default debouncer](https://github.com/zmkfirmware/zmk/blob/19613128b901723f7b78c136792d72e6ca7cf4fc/app/module/lib/zmk_debounce/debounce.c)
pub(crate) struct DefaultDebouncer<const INPUT_PIN_NUM: usize, const OUTPUT_PIN_NUM: usize> {
  last_ms: u32,
  counters: [[Debounce; INPUT_PIN_NUM]; OUTPUT_PIN_NUM],
}

impl<const INPUT_PIN_NUM: usize, const OUTPUT_PIN_NUM: usize> DebouncerTrait
  for DefaultDebouncer<INPUT_PIN_NUM, OUTPUT_PIN_NUM>
{
  /// Create a default debouncer
  fn new() -> Self {
    DefaultDebouncer {
      counters: [[Debounce(0); INPUT_PIN_NUM]; OUTPUT_PIN_NUM],
      last_ms: 0,
    }
  }

  /// Per-key debounce, same with zmk's debounce algorithm
  fn detect_change_with_debounce(
    &mut self,
    in_idx: usize,
    out_idx: usize,
    pin_state: bool,
    key_state: &KeyState,
  ) -> DebounceState {
    // Check debounce state every 1 ms
    let cur_ms = Instant::now().as_millis() as u32;
    let elapsed_ms = (cur_ms - self.last_ms) as u16;

    // If `elapsed_ms` == 0, the debounce state is checked within 1 ms, skip
    if elapsed_ms > 0 {
      let counter: &mut Debounce = &mut self.counters[out_idx][in_idx];

      if key_state.pressed == pin_state {
        // If current key state matches input level, decrease debounce counter
        counter.decrease(elapsed_ms);
        // If there's no key change, the counter should always be 0.
        // So if the counter != 0, it's in a debouncing process
        if counter.0 > 0 {
          DebounceState::InProgress
        } else {
          DebounceState::Ignored
        }
      } else if counter.0 < DEBOUNCE_THRESHOLD {
        // If debounce threshold is not exceeded, increase debounce counter
        counter.increase(elapsed_ms);
        DebounceState::InProgress
      } else {
        // Debounce threshold is exceeded, reset counter
        self.last_ms = cur_ms;
        counter.0 = 0;
        DebounceState::Debounced
      }
    } else {
      DebounceState::Ignored
    }
  }
}

/// Fast per-key debouncer.
/// The debouncing algorithm is similar as QMK's [asym eager defer pk debouncer](https://github.com/qmk/qmk_firmware/blob/2fd56317763e8b3b73f0db7488ef42a70f5b946e/quantum/debounce/asym_eager_defer_pk.c)
pub(crate) struct RapidDebouncer<const INPUT_PIN_NUM: usize, const OUTPUT_PIN_NUM: usize> {
  last_ms: Instant,
  debouncing: [[bool; INPUT_PIN_NUM]; OUTPUT_PIN_NUM],
}

impl<const INPUT_PIN_NUM: usize, const OUTPUT_PIN_NUM: usize> DebouncerTrait
  for RapidDebouncer<INPUT_PIN_NUM, OUTPUT_PIN_NUM>
{
  /// Create a rapid debouncer
  fn new() -> Self {
    RapidDebouncer {
      debouncing: [[false; INPUT_PIN_NUM]; OUTPUT_PIN_NUM],
      last_ms: Instant::now(),
    }
  }

  /// Per-key fast debounce
  fn detect_change_with_debounce(
    &mut self,
    in_idx: usize,
    out_idx: usize,
    pin_state: bool,
    key_state: &KeyState,
  ) -> DebounceState {
    let debouncing = self.debouncing[out_idx][in_idx];
    if debouncing {
      // Current key is in debouncing state
      if self.last_ms.elapsed().as_millis() as u16 > DEBOUNCE_THRESHOLD {
        // If the elapsed time > DEBOUNCE_THRESHOLD, reset
        self.debouncing[out_idx][in_idx] = false;
        DebounceState::Ignored
      } else {
        // Still in a debouncing progress
        DebounceState::InProgress
      }
    } else if key_state.pressed != pin_state {
      // If current key isn't in debouncing state, and a key change is detected
      // Trigger the key imeediately and record current tick
      self.last_ms = Instant::now();
      // Change debouncing state
      self.debouncing[out_idx][in_idx] = true;
      DebounceState::Debounced
    } else {
      DebounceState::Ignored
    }
  }
}
