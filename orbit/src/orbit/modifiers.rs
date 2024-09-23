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

pub fn lc(code: u16) -> u16 {
  code | Modifier::LeftControl as u16
}

pub fn rc(code: u16) -> u16 {
  code | Modifier::RightControl as u16
}

pub fn r(code: u16) -> u16 {
  lc(rc(code))
}

pub fn ls(code: u16) -> u16 {
  code | Modifier::LeftShift as u16
}

pub fn rs(code: u16) -> u16 {
  code | Modifier::RightShift as u16
}

pub fn s(code: u16) -> u16 {
  ls(rs(code))
}

pub fn la(code: u16) -> u16 {
  code | Modifier::LeftAlt as u16
}

pub fn ra(code: u16) -> u16 {
  code | Modifier::RightAlt as u16
}

pub fn a(code: u16) -> u16 {
  la(ra(code))
}

pub fn lg(code: u16) -> u16 {
  code | Modifier::LeftGui as u16
}

pub fn rg(code: u16) -> u16 {
  code | Modifier::RightGui as u16
}

pub fn g(code: u16) -> u16 {
  lg(rg(code))
}
