use uefi::prelude::*;
use alloc::vec::Vec;

const EFI_PAGE_SIZE: u64 = 0x1000;

pub fn memory_map(bt: &BootServices) {
  // Get the estimated map size
  let map_size = bt.memory_map_size();

  // Build a buffer bigger enough to handle the memory map
  let mut buffer = Vec::with_capacity(map_size);
  unsafe {
    buffer.set_len(map_size);
  }

  let (_k, desc_iter) = bt
    .memory_map(&mut buffer)
    .expect_success("Failed to retrieve UEFI memory map");

  assert!(desc_iter.len() > 0, "Memory map is empty");

  // Print out a list of all the usable memory we see in the memory map.
  // Don't print out everything, the memory map is probably pretty big
  // (e.g. OVMF under QEMU returns a map with nearly 50 entries here).
  log::info!("efi: usable memory ranges ({} total)", desc_iter.len());
  for (i, descriptor) in desc_iter.enumerate() {
    let size = descriptor.page_count * EFI_PAGE_SIZE;
    let start_address = descriptor.phys_start;
    let end_address = descriptor.phys_start + size;
    let descriptor_type = descriptor.ty;

    log::info!(" {}) {:?}: {:#x} - {:#x} ({} KiB)", i, descriptor_type, start_address, end_address, size);
  }
}
