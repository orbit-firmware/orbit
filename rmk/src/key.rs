// use crate::time;

// #[derive(Copy, Clone)]
// pub struct Key {
//   pub index: usize,
//   pub state: bool,
//   pub press_time: u32,
// }

// impl Key {
//   pub fn new(index: usize) -> Key {
//     Key {
//       index,
//       state: false,
//       press_time: 0,
//     }
//   }

//   pub fn press(&mut self) {
//     self.state = true;
//     self.press_time = time::now();
//   }

//   pub fn release(&mut self) {
//     self.state = false;
//     self.press_time = 0;
//   }

//   pub fn get_held_time(&self) -> u32 {
//     time::elapsed(self.press_time)
//   }

//   pub fn is_pressed(&self) -> bool {
//     self.state
//   }

//   pub fn is_released(&self) -> bool {
//     !self.state
//   }
// }

// pub fn create<const KEY_COUNT: usize>() -> [Key; KEY_COUNT] {
//   assert!(KEY_COUNT > 0);

//   let mut keys: [Key; KEY_COUNT] = [Key::new(0); KEY_COUNT];

//   for i in 0..KEY_COUNT {
//     keys[i] = Key::new(i);
//   }

//   keys
// }
