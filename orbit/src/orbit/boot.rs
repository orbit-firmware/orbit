// the boot key: a magic word in RAM that survives a reset makes the next start jump into
// the chip's ROM bootloader (USB DFU, flash with dfu-util) before anything is initialised
use core::mem::MaybeUninit;
use core::ptr::{addr_of_mut, read_volatile, write_volatile};

use crate::orbit::config as Orbit;

const MAGIC: u32 = 0xB007_B007;

#[link_section = ".uninit.BOOT"]
static mut FLAG: MaybeUninit<u32> = MaybeUninit::uninit();

pub fn enter() -> ! {
  unsafe { write_volatile(addr_of_mut!(FLAG) as *mut u32, MAGIC) };
  cortex_m::peripheral::SCB::sys_reset()
}

#[cortex_m_rt::pre_init]
unsafe fn jump_to_bootloader() {
  let flag = addr_of_mut!(FLAG) as *mut u32;
  if read_volatile(flag) == MAGIC {
    write_volatile(flag, 0);
    // system memory, where the ROM bootloader's vector table sits
    let rom: u32 = if Orbit::CHIP.starts_with("stm32f303") { 0x1FFF_D800 } else { 0x1FFF_0000 };
    cortex_m::asm::bootload(rom as *const u32)
  }
}
