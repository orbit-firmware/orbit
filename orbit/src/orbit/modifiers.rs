#![allow(dead_code)]

// Mod bits:    43210
//    bit 4      +----- LR flag (Left:0, Right:1)
//    bit 3      |+---- Gui
//    bit 2      ||+--- Alt
//    bit 1      |||+-- Shift
//    bit 0      ||||+- Control
#[repr(u16)]
pub enum Modifier {
  LeftControl = 0x0100,
  RightControl = 0x1100,
  LeftShift = 0x0200,
  RightShift = 0x1200,
  LeftAlt = 0x0400,
  RightAlt = 0x1400,
  LeftGui = 0x0800,
  RightGui = 0x1800,
}

pub fn lc(keycode: u16) -> u16 {
  keycode | Modifier::LeftControl as u16
}

pub fn rc(keycode: u16) -> u16 {
  keycode | Modifier::RightControl as u16
}

pub fn r(keycode: u16) -> u16 {
  lc(rc(keycode))
}

pub fn ls(keycode: u16) -> u16 {
  keycode | Modifier::LeftShift as u16
}

pub fn rs(keycode: u16) -> u16 {
  keycode | Modifier::RightShift as u16
}

pub fn s(keycode: u16) -> u16 {
  ls(rs(keycode))
}

pub fn la(keycode: u16) -> u16 {
  keycode | Modifier::LeftAlt as u16
}

pub fn ra(keycode: u16) -> u16 {
  keycode | Modifier::RightAlt as u16
}

pub fn a(keycode: u16) -> u16 {
  la(ra(keycode))
}

pub fn lg(keycode: u16) -> u16 {
  keycode | Modifier::LeftGui as u16
}

pub fn rg(keycode: u16) -> u16 {
  keycode | Modifier::RightGui as u16
}

pub fn g(keycode: u16) -> u16 {
  lg(rg(keycode))
}

pub fn has_modifier(keycode: u16) -> bool {
  keycode & 0xFF00 != 0
}

pub fn get_modifier(keycode: u16) -> u16 {
  keycode & 0xFF00
}

pub fn get_modifier_u8(keycode: u16) -> u8 {
  (keycode >> 8) as u8
}
