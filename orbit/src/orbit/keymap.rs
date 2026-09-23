// the keymap tables get generated from build/keymap.orbit by
// orbit/macros/src/generators/keymap.rs

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Entry {
  // falls through to the next lower active layer
  Trough,
  None,
  // keycode, high byte = hid modifier bits
  Code(u16),
  // momentary layer while held
  Layer(u8),
  // switches the base layer
  To(u8),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Slot {
  pub press: Entry,
  // replaces press while shift is held; Trough = no replacement
  pub shift: Entry,
  // sent instead of press when held for hold_ms; Trough = no hold
  pub hold: Entry,
  pub hold_ms: u16,
}

pub struct Combo {
  pub keys: &'static [usize],
  pub entry: Entry,
}

orbit_macros::generate_keymap! {}
