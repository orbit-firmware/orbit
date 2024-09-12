// list of behaviors
#[derive(Debug)]
#[repr(u8)]
pub enum Behaviors {
  Press, // press is always enabled
  Hold,
  Tap,
  Modding,
}

pub trait Behavior {
  fn process(&self);
}

mod hold;
mod modding;
mod press;
mod tap;

pub fn process() {}
