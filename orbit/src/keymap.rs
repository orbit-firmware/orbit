// use crate::orbit::{
//   behaviors::Behaviors,
//   config::{BEHAVIOR_COUNT, KEY_COUNT},
// };

// pub struct KeyMap<const LAYER_COUNT: usize> {
//   pub layers: [[[Behaviors; BEHAVIOR_COUNT]; KEY_COUNT]; LAYER_COUNT],
//   layer_state: [bool; LAYER_COUNT],
//   default_layer: u8,
//   layer_cache: [u8; KEY_COUNT],
// }

// impl<const LAYER_COUNT: usize> KeyMap<LAYER_COUNT> {
//   // pub async fn create<F: NorFlash>(
//   //   mut action_map: [[Behavior;; KEY_COUNT]; LAYER_COUNT],
//   //   storage: Option<&mut Storage<F>>,
//   // ) -> Self {
//   //   //   // If the storage is initialized, read keymap from storage
//   //   //   let mut macro_cache = [0; MACRO_SPACE_SIZE];
//   //   //   if let Some(storage) = storage {
//   //   //     // Read keymap to `action_map`
//   //   //     if storage.read_keymap(&mut action_map).await.is_err() {
//   //   //       error!("Keymap reading aborted by an error, clearing the storage...");
//   //   //       // Dont sent flash message here, since the storage task is not running yet
//   //   //       sequential_storage::erase_all(&mut storage.flash, storage.storage_range.clone())
//   //   //         .await
//   //   //         .ok();

//   //   //       reboot_keyboard();
//   //   //     } else {
//   //   //       // Read macro cache
//   //   //       if let Err(_) = storage
//   //   //         .read_macro_cache::<ROW, COL, NUM_LAYER>(&mut macro_cache)
//   //   //         .await
//   //   //       {
//   //   //         error!("Wrong macro cache, clearing the storage...");
//   //   //         sequential_storage::erase_all(&mut storage.flash, storage.storage_range.clone())
//   //   //           .await
//   //   //           .ok();

//   //   //         reboot_keyboard();
//   //   //       }
//   //   //     }
//   //   //   }

//   //   //   KeyMap {
//   //   //     layers: action_map,
//   //   //     layer_state: [false; NUM_LAYER],
//   //   //     default_layer: 0,
//   //   //     layer_cache: [[0; COL]; ROW],
//   //   //     macro_cache,
//   //   //   }
//   // }
// }

// // pub fn get_key_chain(layer: usize, row: usize, col: usize, feature: usize) -> KeyChain {
// //   assert!(layer < 2_usize.pow(LAYER_BITS as u32));
// //   assert!(row < 2_usize.pow(ROW_BITS as u32));
// //   assert!(col < 2_usize.pow(COL_BITS as u32));
// //   assert!(feature < 2_usize.pow(FEATURE_BITS as u32));

// //   let index = (layer << (ROW_BITS + COL_BITS + FEATURE_BITS))
// //     | (row << (COL_BITS + FEATURE_BITS))
// //     | (col << FEATURE_BITS)
// //     | feature;

// //   let mut low = 0;
// //   let mut high = KEY_INDECIES.len();

// //   while low < high {
// //     let mid = (low + high) / 2;
// //     let mid_val = KEY_INDECIES[mid];

// //     if mid_val < index as u32 {
// //       low = mid + 1;
// //     } else if mid_val > index as u32 {
// //       high = mid;
// //     } else {
// //       return KEY_CODES[mid];
// //     }
// //   }

// //   [0; KEY_CHAIN_LENGTH]
// // }
