#![feature(abi_x86_interrupt)]
#![feature(core_intrinsics)]
#![feature(alloc_layout_extra)]
#![feature(custom_inner_attributes)]
#![feature(const_in_array_repeat_expressions)]
#![feature(allocator_api)]
#![no_std]

extern crate alloc;

#[macro_use]
extern crate lazy_static;

mod boot_info;
mod device;
mod error;
mod event_buffer;
mod file;
mod interrupts;
#[macro_use] pub mod logging;
mod memory;

use kernel::{Platform};
use self::{
  device::{
    pc_keyboard::PCKeyboard,
    Device
  },
  error::X8664Error,
  file::X8664File
};

type PlatformEvent = kernel::PlatformEvent::<X8664Platform>;

pub use boot_info::{X8664BootInfo, X8664MemorySegment};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DeviceID {
  PCKeyboard,
  UEFIFilesystem
}

#[derive(Clone)]
pub struct X8664Platform {
  boot_info: X8664BootInfo
}

impl X8664Platform {
  pub fn early_init() {
    logging::init();
  }

  pub fn new(boot_info: X8664BootInfo) -> Self {
    Self { boot_info }
  }

  fn init_allocator(&self) {
    let mut available_memory = 0;

    for segment in self.boot_info.memory_map.iter() {
      memory::allocator::get().lock().add_heap(segment.start_address, segment.length);
      available_memory += segment.length
    }    

    log::info!("Memory allocator configured with {} MiB", available_memory / 1_048_576);

  }

  fn init_interrupts(&self) {
    interrupts::init();
    log::info!("Interrupts configured");

  }
}

impl Platform for X8664Platform {
  type DeviceID = DeviceID;
  type Device = Device;
  type Error = X8664Error;
  type File = X8664File;

  fn init(&mut self) {   
    self.init_allocator(); 
    self.init_interrupts();

    event_buffer::push_event(PlatformEvent::DeviceConnected(
      DeviceID::PCKeyboard, 
      Device::PCKeyboard(PCKeyboard::new())
    ));

    log::info!("Done!");
  }

  fn poll_event(&self) -> Option<PlatformEvent> {
    event_buffer::poll_event()
  }

  fn sleep(&self) {
    x86_64::instructions::hlt()
  }
}
