#[allow(dead_code)]
#[repr(u8)]
pub enum Actions {
  Mouse,
}

#[cfg(feature = "action_mouse_enabled")]
mod mouse;

pub fn process() {}
