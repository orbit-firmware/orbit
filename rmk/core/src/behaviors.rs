// list of behaviors
#[derive(Debug)]
#[repr(u8)]
pub enum Behaviors {
  Press, // press is always enabled
  Hold,
  Tap,
  Modding,
}

pub struct Behavior {}
