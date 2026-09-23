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
  // turns a layer on until pressed again
  Toggle(u8),
  // tapped: applies to the next key press only; held: a normal key
  Sticky(u16),
  StickyLayer(u8),
  // letters shifted until a non-word key
  CapsWord,
  // the last sent keycode again
  Repeat,
  // restarts into the chip's bootloader
  Boot,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Slot {
  pub press: Entry,
  // replaces press while shift is held; Trough = no replacement
  pub shift: Entry,
  // sent instead of press when held for hold_ms; Trough = no hold
  pub hold: Entry,
  pub hold_ms: u16,
  // sent instead of press on a second tap within the tapping term; Trough = none
  pub tap2: Entry,
}

pub struct Combo {
  pub keys: &'static [usize],
  pub entry: Entry,
}

orbit_macros::generate_keymap! {}
