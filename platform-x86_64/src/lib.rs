#![feature(abi_x86_interrupt)]
#![feature(core_intrinsics)]
#![feature(alloc_layout_extra)]
#![no_std]

extern crate alloc;

#[macro_use]
extern crate lazy_static;

mod event_buffer;
mod file;
mod interrupts;
mod device;
mod error;
//mod memory;

use alloc::{
  rc::Rc
};
use core::cell::RefCell;

use uefi::prelude::*;

use kernel::{Platform};
use self::{
  device::{
    pc_keyboard::PCKeyboard,
    uefi_filesystem::UEFIFilesystem,
    Device
  },
  error::X8664Error,
  file::X8664File
};

type PlatformEvent = kernel::PlatformEvent::<X8664Platform>;


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DeviceID {
  PCKeyboard,
  UEFIFilesystem
}

#[derive(Clone)]
pub struct X8664Platform {
  system_table: Rc<RefCell<SystemTable<Boot>>>,
}

impl X8664Platform {
  pub fn new(system_table: SystemTable<Boot>) -> Self {
    Self { 
      system_table: Rc::new(RefCell::new(system_table))
    }
  }

  fn init_interrupts(&self) {
    interrupts::init();
  }
}

impl Platform for X8664Platform {
  type DeviceID = DeviceID;
  type Device = Device;
  type Error = X8664Error;
  type File = X8664File;

  fn init(&mut self) {

    // Print out the UEFI revision number
    {
      let system_table = self.system_table.borrow();
      let rev = system_table.uefi_revision();
      let (major, minor) = (rev.major(), rev.minor());

      log::info!("Booted by UEFI {}.{}", major, minor);
    }

    log::info!("Initializing...");

    x86_64::instructions::interrupts::disable();

    log::debug!("- Interrupts");
    self.init_interrupts();

    x86_64::instructions::interrupts::enable();

    event_buffer::push_event(PlatformEvent::DeviceConnected(
      DeviceID::PCKeyboard, 
      Device::PCKeyboard(PCKeyboard::new())
    ));

    event_buffer::push_event(PlatformEvent::DeviceConnected(
      DeviceID::UEFIFilesystem, 
      Device::UEFIFilesystem(UEFIFilesystem::new(&self.system_table))
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
