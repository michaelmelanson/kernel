#![no_std]
#![no_main]

#[macro_use] extern crate platform_x86_64;

extern crate alloc;

use uefi::{
  prelude::*,
  table::boot::MemoryType
};
use kernel::Kernel;
use platform_x86_64::{
  X8664Platform,
  X8664BootInfo,
  X8664MemorySegment
};

#[no_mangle]
pub extern "win64" fn uefi_start(image: uefi::Handle, system_table: SystemTable<Boot>) -> ! {
  X8664Platform::early_init();
      
  // Print out the UEFI revision number
  {
    let rev = system_table.uefi_revision();
    let (major, minor) = (rev.major(), rev.minor());

    log::info!("Booted by UEFI {}.{}", major, minor);
  }

  const MAX_MMAP_SIZE: usize = 103680;
  let estimated_mmap_size = system_table.boot_services().memory_map_size();

  if estimated_mmap_size > MAX_MMAP_SIZE {
    panic!("Memory map is too large");
  }

  let mut uefi_mmap_storage = [0; MAX_MMAP_SIZE];
  let (system_table, uefi_memory_map_iter) = system_table
    .exit_boot_services(image, &mut uefi_mmap_storage[..])
    .expect_success("Failed to exit boot services");

  let mut memory_map = [X8664MemorySegment { start_address: 0, length: 0}; 100];
  let mut memory_map_segments = 0;

  for (i, descriptor) in uefi_memory_map_iter.enumerate() {
    let size = descriptor.page_count * 0x1000;
    let start_address = descriptor.phys_start;
    let end_address = descriptor.phys_start + size;
    let descriptor_type = descriptor.ty;

    match descriptor_type {
      MemoryType::CONVENTIONAL | MemoryType::BOOT_SERVICES_CODE | MemoryType::BOOT_SERVICES_DATA => {
        memory_map[memory_map_segments] = X8664MemorySegment {
          start_address, length: size
        };
        memory_map_segments += 1;
      },

      _ => {}
    }

  }

  let boot_info = X8664BootInfo { memory_map };
  Kernel::new(X8664Platform::new(boot_info)).start()
}