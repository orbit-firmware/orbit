// Turns debounced key states into boot reports through the keymap:
// layers, shift replacement, hold-tap, tap dance, combos, sticky keys, caps word and repeat.

use heapless::Vec;

use crate::orbit::config as Orbit;
use crate::orbit::keymap::{Entry, Slot, COMBOS, COMBO_TERM, KEYMAP, LAYER_COUNT, QUICK_TAP_TERM};

const SHIFT_BITS: u8 = 0x22;
const LEFT_SHIFT: u8 = 0xE1;
const RIGHT_SHIFT: u8 = 0xE5;
const LEFT_SHIFT_BIT: u8 = 0x02;

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

// sk(key) / skl(n): held like a normal key; released without another key going down it
// arms, and applies to the next key press until that key is released
#[derive(Clone, Copy, PartialEq)]
enum Sticky {
  Off,
  Held { k: usize, entry: Entry, interrupted: bool },
  Armed(Entry),
  Applied { k: usize, entry: Entry },
}

pub struct Engine {
  keys: [State; Orbit::KEY_COUNT],
  base: u8,
  // entries sent for a single report (taps)
  taps: Vec<(Entry, bool), 8>,
  waiting: Vec<(usize, u32), 8>,
  // the active combo and what it sends
  combo: Option<(usize, Entry)>,
  // layers turned on by tl(n)
  toggled: u32,
  sticky: Sticky,
  caps_word: bool,
  // the last sent keycode, for rep
  last: Option<u16>,
  // the last hold-tap key tapped and when, for quick tap
  tapped: Option<(usize, u32)>,
  // a tap dance key released once: its tap waits for a second press until the tapping term
  dance: Option<(usize, u32, Entry, bool)>,
  // a boot key was pressed
  pub boot: bool,
}

impl Engine {
  pub fn new() -> Engine {
    Engine {
      keys: [State::Up; Orbit::KEY_COUNT],
      base: 0,
      taps: Vec::new(),
      waiting: Vec::new(),
      combo: None,
      toggled: 0,
      sticky: Sticky::Off,
      caps_word: false,
      last: None,
      tapped: None,
      dance: None,
      boot: false,
    }
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
    if let Some((_, at, ..)) = self.dance {
      if now.wrapping_sub(at) >= Orbit::TAPPING_TERM as u32 {
        self.end_dance();
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
          self.hold(k, slot);
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
        self.resolve_pending();
        let entry = self.effect(None, COMBOS[i].entry);
        self.combo = Some((i, entry));
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
      // released before the combo term: its press is still sent, as a tap
      if let State::Active { entry, replaced } = self.keys[k] {
        let _ = self.taps.push((entry, replaced));
      }
    }
    if let Sticky::Held { k: held, entry, interrupted } = self.sticky {
      if held == k {
        self.sticky = if interrupted { Sticky::Off } else { Sticky::Armed(entry) };
      }
    }
    match self.keys[k] {
      State::Pending { slot, .. } => {
        let (entry, replaced) = self.pick(&slot);
        if slot.tap2 != Entry::Trough {
          self.dance = Some((k, now, entry, replaced));
        } else {
          let entry = self.effect(None, entry);
          let _ = self.taps.push((entry, replaced));
        }
        self.tapped = Some((k, now));
      }
      State::Consumed if self.combo.is_some_and(|(c, _)| COMBOS[c].keys.contains(&k)) => self.combo = None,
      _ => {}
    }
    if let Sticky::Applied { k: applied, entry } = self.sticky {
      if applied == k {
        // a tap goes out in this report: the sticky entry goes with it
        if !self.taps.is_empty() {
          let _ = self.taps.push((entry, false));
        }
        self.sticky = Sticky::Off;
      }
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
    // the second press of a tap dance
    if self.dance.is_some_and(|(d, ..)| d == k) && slot.tap2 != Entry::Trough {
      self.dance = None;
      return self.activate(k, slot.tap2, false);
    }
    self.end_dance();
    match self.sticky {
      Sticky::Held { k: held, entry, .. } => self.sticky = Sticky::Held { k: held, entry, interrupted: true },
      // a second sticky key cancels the armed one
      Sticky::Armed(_) if matches!(slot.press, Entry::Sticky(_) | Entry::StickyLayer(_)) => {
        self.sticky = Sticky::Off;
        self.keys[k] = State::Active { entry: Entry::None, replaced: false };
        return;
      }
      // an armed layer only picks this key's slot
      Sticky::Armed(Entry::Layer(_)) => self.sticky = Sticky::Off,
      Sticky::Armed(entry) => self.sticky = Sticky::Applied { k, entry },
      _ => {}
    }
    // quick tap: pressed again right after a tap, a hold-tap key holds its press
    let quick = self.tapped.is_some_and(|(t, at)| t == k && now.wrapping_sub(at) < QUICK_TAP_TERM as u32);
    if (slot.hold != Entry::Trough || slot.tap2 != Entry::Trough) && !quick {
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
        self.hold(k, slot);
      }
    }
  }

  // a pending key held long enough, or interrupted: its hold, else (tap dance only) its press
  fn hold(&mut self, k: usize, slot: Slot) {
    if slot.hold != Entry::Trough {
      self.activate(k, slot.hold, false);
    } else {
      let (entry, replaced) = self.pick(&slot);
      self.activate(k, entry, replaced);
    }
  }

  // a waiting single tap goes out
  fn end_dance(&mut self) {
    if let Some((_, _, entry, replaced)) = self.dance.take() {
      let entry = self.effect(None, entry);
      let _ = self.taps.push((entry, replaced));
    }
  }

  fn activate(&mut self, k: usize, entry: Entry, replaced: bool) {
    let entry = self.effect(Some(k), entry);
    self.keys[k] = State::Active { entry, replaced };
  }

  // what an entry does when it goes down (held by key k, or tapped when None); returns
  // what it sends from then on
  fn effect(&mut self, k: Option<usize>, entry: Entry) -> Entry {
    let entry = match entry {
      Entry::Repeat => self.last.map_or(Entry::None, Entry::Code),
      _ => entry,
    };
    match entry {
      Entry::To(l) => self.base = l,
      Entry::Toggle(l) => self.toggled ^= 1 << l,
      Entry::CapsWord => self.caps_word = !self.caps_word,
      Entry::Boot => self.boot = true,
      Entry::Code(c) => self.typed(c),
      Entry::Sticky(c) => return self.stick(k, Entry::Code(c)),
      Entry::StickyLayer(l) => return self.stick(k, Entry::Layer(l)),
      _ => {}
    }
    entry
  }

  fn stick(&mut self, k: Option<usize>, held: Entry) -> Entry {
    self.sticky = match k {
      Some(k) => Sticky::Held { k, entry: held, interrupted: false },
      None => Sticky::Armed(held),
    };
    held
  }

  // a keycode went down: remember it for rep, end caps word on a non-word key
  fn typed(&mut self, code: u16) {
    let key = code as u8;
    if (0xE0..=0xE7).contains(&key) {
      return;
    }
    self.last = Some(code);
    // letters, digits, -, backspace, delete
    let word = matches!(key, 0x04..=0x27 | 0x2D | 0x2A | 0x4C);
    if !word {
      self.caps_word = false;
    }
  }

  // the slot of the highest active layer that does not fall through
  fn slot(&self, k: usize) -> Slot {
    let mut layers: u32 = 1 | 1 << self.base | self.toggled;
    if let Sticky::Armed(Entry::Layer(l)) = self.sticky {
      layers |= 1 << l;
    }
    for state in self.keys.iter() {
      if let State::Active { entry: Entry::Layer(l), .. } = state {
        layers |= 1 << l;
      }
    }
    if let Some((_, Entry::Layer(l))) = self.combo {
      layers |= 1 << l;
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
    let sticky = match self.sticky {
      Sticky::Armed(e) | Sticky::Applied { entry: e, .. } => Some(e),
      _ => None,
    };
    let held = self.keys.iter().filter_map(|s| match s {
      State::Active { entry, replaced: false } => Some(*entry),
      _ => None,
    });
    held.chain(sticky).any(|e| match e {
      Entry::Code(c) => {
        let key = c as u8;
        (c >> 8) as u8 & SHIFT_BITS != 0 || key == LEFT_SHIFT || key == RIGHT_SHIFT
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
    let combo = self.combo.map(|(_, entry)| (entry, false));
    let sticky = match self.sticky {
      Sticky::Applied { entry, .. } => Some((entry, false)),
      _ => None,
    };
    for (entry, replaced) in held.chain(combo).chain(sticky).chain(self.taps.iter().copied()) {
      let Entry::Code(code) = entry else { continue };
      let mut mods = (code >> 8) as u8;
      let mut key = code as u8;
      if (0xE0..=0xE7).contains(&key) {
        // modifier keys are bits of the modifier byte, not array entries
        mods |= 1 << (key - 0xE0);
        key = 0;
      }
      // caps word shifts letters and turns - into _
      if self.caps_word && matches!(key, 0x04..=0x1D | 0x2D) {
        mods |= LEFT_SHIFT_BIT;
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
