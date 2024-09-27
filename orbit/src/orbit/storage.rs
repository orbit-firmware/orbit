use crate::orbit::config as Orbit;
// use usbd_mass_storage::{BlockDevice, UsbMassStorage};

// pub struct StorageBlock {
//   storage: [u8; Orbit::STORAGE * 1024],
// }

// impl StorageBlock {
//   pub fn new() -> Self {
//     Self {
//       storage: [0; Orbit::STORAGE * 1024],
//     }
//   }
// }

// impl BlockDevice for StorageBlock {
//   const BLOCK_SIZE: u16 = Orbit::STORAGE;

//   fn read_block(&mut self, block: u32, buf: &mut [u8]) {
//     let start = (block as usize) * Self::BLOCK_SIZE as usize;
//     let end = start + Self::BLOCK_SIZE as usize;
//     buf.copy_from_slice(&self.storage[start..end]);
//   }

//   fn write_block(&mut self, block: u32, buf: &[u8]) {
//     let start = (block as usize) * Self::BLOCK_SIZE as usize;
//     let end = start + Self::BLOCK_SIZE as usize;
//     self.storage[start..end].copy_from_slice(buf);
//   }
// }

pub struct Storage {
  //   block: StorageBlock,
  //   fs: FileSystem,
}

impl Storage {
  pub fn init() {
    // -> Self {
    // let block = StorageBlock::new();
    // let fs = FileSystem::new(&mut block);
    // Self { block, fs }
  }

  // pub fn fs(&mut self) -> &mut FileSystem {
  //   &mut self.fs
  // }
}
