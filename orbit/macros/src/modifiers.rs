#![allow(dead_code)]
// The high byte is the HID modifier byte, sent as is:
//   bit 0-3: left  Control, Shift, Alt, Gui
//   bit 4-7: right Control, Shift, Alt, Gui
#[repr(u16)]
pub enum Modifier {
  LeftControl = 0x0100,
  RightControl = 0x1000,
  LeftShift = 0x0200,
  RightShift = 0x2000,
  LeftAlt = 0x0400,
  RightAlt = 0x4000,
  LeftGui = 0x0800,
  RightGui = 0x8000,
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

#[cfg(test)]
mod tests {
  use super::*;

  // the high byte must match the HID boot report modifier bits
  #[test]
  fn high_byte_is_hid_modifier_byte() {
    assert_eq!(lc(0) >> 8, 0x01);
    assert_eq!(ls(0) >> 8, 0x02);
    assert_eq!(la(0) >> 8, 0x04);
    assert_eq!(lg(0) >> 8, 0x08);
    assert_eq!(rc(0) >> 8, 0x10);
    assert_eq!(rs(0) >> 8, 0x20);
    assert_eq!(ra(0) >> 8, 0x40);
    assert_eq!(rg(0) >> 8, 0x80);
    assert_eq!(s(0x05), 0x2205);
  }
}
