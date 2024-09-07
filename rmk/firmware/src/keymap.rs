// use crate::keycodes::{KeyChain, KEY_CHAIN_LENGTH, KEY_CODES, KEY_INDECIES};

// const LAYER_BITS: u16 = 5; // For up to 32 layers (2^5)
// const ROW_BITS: u16 = 5; // For up to 32 rows (2^5)
// const COL_BITS: u16 = 5; // For up to 32 columns (2^5)
// const FEATURE_BITS: u16 = 5; // For up to 32 features (2^5)

// #[rustfmt::skip]
// pub fn get_key_chain(layer: usize, row: usize, col: usize, feature: usize) -> KeyChain {
//   assert!(layer < 2_usize.pow(LAYER_BITS as u32));
//   assert!(row < 2_usize.pow(ROW_BITS as u32));
//   assert!(col < 2_usize.pow(COL_BITS as u32));
//   assert!(feature < 2_usize.pow(FEATURE_BITS as u32));

//   let index = (layer << (ROW_BITS + COL_BITS + FEATURE_BITS)) |
//               (row << (COL_BITS + FEATURE_BITS)) |
//               (col << FEATURE_BITS) |
//               feature;

//   let mut low = 0;
//   let mut high = KEY_INDECIES.len();

//   while low < high {
//       let mid = (low + high) / 2;
//       let mid_val = KEY_INDECIES[mid];

//       if mid_val < index as u32 {
//           low = mid + 1;
//       } else if mid_val > index as u32 {
//           high = mid;
//       } else {
//           return KEY_CODES[mid];
//       }
//   }

//   [0; KEY_CHAIN_LENGTH]
// }
