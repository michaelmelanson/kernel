#![no_std]
#![no_main]

#[macro_use]
extern crate log;

extern crate alloc;
extern crate uefi;
extern crate uefi_services;

extern crate kernel;
extern crate platform_x86_64;
extern crate ringbuffer;

use uefi::prelude::*;
use kernel::Kernel;
use platform_x86_64::X8664Platform;

mod memory;

#[no_mangle]
pub extern "win64" fn uefi_start(_image_handle: uefi::Handle, system_table: &'static SystemTable) -> ! {
  // Initialize logging.
  uefi_services::init(system_table);

  // Print out the UEFI revision number
  {
      let rev = system_table.uefi_revision();
      let (major, minor) = (rev.major(), rev.minor());

      info!("Booted by UEFI {}.{}", major, minor);
  }

  memory::memory_map(&system_table.boot);

  Kernel::new(X8664Platform::new()).start()
}