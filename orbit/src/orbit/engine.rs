// Turns debounced key states into boot reports through the keymap:
// layers, shift replacement, hold-tap and combos.

use heapless::Vec;

use crate::orbit::config as Orbit;
use crate::orbit::keymap::{Entry, Slot, COMBOS, COMBO_TERM, KEYMAP, LAYER_COUNT};

const SHIFT_BITS: u8 = 0x22;
const LEFT_SHIFT: u8 = 0xE1;
const RIGHT_SHIFT: u8 = 0xE5;

pub struct Report {
  pub modifier: u8,
  pub keycodes: [u8; 6],
}

impl Report {
  pub fn serialize(&self) -> [u8; 8] {
    let mut buf = [0; 8];
    buf[0] = self.modifier;
    buf[2..8].copy_from_slice(&self.keycodes);
    buf
  }
}

#[derive(Clone, Copy, PartialEq)]
enum State {
  Up,
  // held, waiting to see if it becomes a combo
  Waiting,
  // hold-tap: becomes the hold after hold_ms or when another key is pressed,
  // the press (a tap) when released before that
  Pending { since: u32, slot: Slot },
  // sends entry until released; replaced = it came from a shift row
  Active { entry: Entry, replaced: bool },
  // held as part of a combo
  Consumed,
}

pub struct Engine {
  keys: [State; Orbit::KEY_COUNT],
  base: u8,
  // entries sent for a single report (taps)
  taps: Vec<(Entry, bool), 8>,
  waiting: Vec<(usize, u32), 8>,
  combo: Option<usize>,
}

impl Engine {
  pub fn new() -> Engine {
    Engine { keys: [State::Up; Orbit::KEY_COUNT], base: 0, taps: Vec::new(), waiting: Vec::new(), combo: None }
  }

  pub fn update(&mut self, pressed: &[bool; Orbit::KEY_COUNT], now: u32) -> Report {
    self.taps.clear();
    for k in 0..Orbit::KEY_COUNT {
      if !pressed[k] && self.keys[k] != State::Up {
        self.release(k, now);
      }
    }
    for k in 0..Orbit::KEY_COUNT {
      if pressed[k] && self.keys[k] == State::Up {
        self.press(k, now);
      }
    }
    if let Some(&(_, since)) = self.waiting.first() {
      if now.wrapping_sub(since) >= COMBO_TERM as u32 {
        self.flush(now);
      }
    }
    for k in 0..Orbit::KEY_COUNT {
      if let State::Pending { since, slot } = self.keys[k] {
        if now.wrapping_sub(since) >= slot.hold_ms as u32 {
          self.activate(k, slot.hold, false);
        }
      }
    }
    self.report()
  }

  // what key k sends if pressed now, for display
  pub fn entry(&self, k: usize) -> Entry {
    self.pick(&self.slot(k)).0
  }

  fn press(&mut self, k: usize, now: u32) {
    if COMBOS.iter().any(|c| c.keys.contains(&k)) {
      self.keys[k] = State::Waiting;
      if self.waiting.push((k, now)).is_err() {
        return self.flush(now);
      }
      let held = |c: &&crate::orbit::keymap::Combo| self.waiting.iter().all(|(w, _)| c.keys.contains(w));
      if let Some(i) = COMBOS.iter().position(|c| c.keys.len() == self.waiting.len() && held(&c)) {
        for &(w, _) in self.waiting.iter() {
          self.keys[w] = State::Consumed;
        }
        self.waiting.clear();
        self.combo = Some(i);
        self.resolve_pending();
      } else if !COMBOS.iter().any(|c| held(&c)) {
        self.flush(now);
      }
      return;
    }
    self.flush(now);
    self.start(k, now);
  }

  fn release(&mut self, k: usize, now: u32) {
    if self.keys[k] == State::Waiting {
      self.flush(now);
    }
    match self.keys[k] {
      State::Pending { slot, .. } => {
        let (entry, replaced) = self.pick(&slot);
        if let Entry::To(l) = entry {
          self.base = l;
        }
        let _ = self.taps.push((entry, replaced));
      }
      State::Consumed if self.combo.is_some_and(|c| COMBOS[c].keys.contains(&k)) => self.combo = None,
      _ => {}
    }
    self.keys[k] = State::Up;
  }

  // waiting combo candidates turn into normal presses, in press order
  fn flush(&mut self, now: u32) {
    let waiting = core::mem::take(&mut self.waiting);
    for (k, _) in waiting {
      self.keys[k] = State::Up;
      self.start(k, now);
    }
  }

  fn start(&mut self, k: usize, now: u32) {
    self.resolve_pending();
    let slot = self.slot(k);
    if slot.hold != Entry::Trough {
      self.keys[k] = State::Pending { since: now, slot };
    } else {
      let (entry, replaced) = self.pick(&slot);
      self.activate(k, entry, replaced);
    }
  }

  // another key went down: pending hold-taps become holds
  fn resolve_pending(&mut self) {
    for k in 0..Orbit::KEY_COUNT {
      if let State::Pending { slot, .. } = self.keys[k] {
        self.activate(k, slot.hold, false);
      }
    }
  }

  fn activate(&mut self, k: usize, entry: Entry, replaced: bool) {
    if let Entry::To(l) = entry {
      self.base = l;
    }
    self.keys[k] = State::Active { entry, replaced };
  }

  // the slot of the highest active layer that does not fall through
  fn slot(&self, k: usize) -> Slot {
    let mut layers: u32 = 1 | 1 << self.base;
    for state in self.keys.iter() {
      if let State::Active { entry: Entry::Layer(l), .. } = state {
        layers |= 1 << l;
      }
    }
    (0..LAYER_COUNT)
      .rev()
      .filter(|l| layers & 1 << l != 0)
      .map(|l| KEYMAP[l][k])
      .find(|s| s.press != Entry::Trough)
      .unwrap_or(KEYMAP[0][k])
  }

  fn pick(&self, slot: &Slot) -> (Entry, bool) {
    if slot.shift != Entry::Trough && self.shift_held() {
      (slot.shift, true)
    } else {
      (slot.press, false)
    }
  }

  fn shift_held(&self) -> bool {
    self.keys.iter().any(|s| match s {
      State::Active { entry: Entry::Code(c), replaced: false } => {
        let key = *c as u8;
        (*c >> 8) as u8 & SHIFT_BITS != 0 || key == LEFT_SHIFT || key == RIGHT_SHIFT
      }
      _ => false,
    })
  }

  // ponytail: codes past the 6 keycode slots are dropped, add nkro when that matters
  fn report(&self) -> Report {
    let mut report = Report { modifier: 0, keycodes: [0; 6] };
    let (mut replaced_mods, mut any_replaced, mut n) = (0u8, false, 0);

    let held = self.keys.iter().filter_map(|s| match s {
      State::Active { entry, replaced } => Some((*entry, *replaced)),
      _ => None,
    });
    let combo = self.combo.map(|c| (COMBOS[c].entry, false));
    for (entry, replaced) in held.chain(combo).chain(self.taps.iter().copied()) {
      let Entry::Code(code) = entry else { continue };
      let mut mods = (code >> 8) as u8;
      let mut key = code as u8;
      if (0xE0..=0xE7).contains(&key) {
        // modifier keys are bits of the modifier byte, not array entries
        mods |= 1 << (key - 0xE0);
        key = 0;
      }
      if replaced {
        any_replaced = true;
        replaced_mods |= mods;
      } else {
        report.modifier |= mods;
      }
      if key != 0 && n < report.keycodes.len() && !report.keycodes[..n].contains(&key) {
        report.keycodes[n] = key;
        n += 1;
      }
    }
    // a shift row replaces the key including the shift that selected it
    if any_replaced {
      report.modifier &= !SHIFT_BITS;
    }
    report.modifier |= replaced_mods;
    report
  }
}
