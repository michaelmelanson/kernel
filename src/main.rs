#![no_std]
#![no_main]

extern crate alloc;

use uefi::prelude::*;
use kernel::Kernel;
use platform_x86_64::X8664Platform;

#[no_mangle]
pub extern "win64" fn uefi_start(_image_handle: uefi::Handle, system_table: SystemTable<Boot>) -> ! {
  // Initialize utilities (logging, memory allocation...)
  uefi_services::init(&system_table).expect_success("Failed to initialize utilities");

  Kernel::new(X8664Platform::new(system_table)).start()
}