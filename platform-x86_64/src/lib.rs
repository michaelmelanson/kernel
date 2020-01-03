#![feature(abi_x86_interrupt)]
#![feature(core_intrinsics)]
#![no_std]

extern crate alloc;

#[macro_use]
extern crate lazy_static;

mod event_buffer;
mod interrupts;
mod device;
//mod memory;

use alloc::vec::Vec;

use uefi::prelude::*;

use kernel::{Platform, PlatformEvent};
use self::{
  device::Device
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DeviceID {
  PCKeyboard
}

pub struct X8664Platform {
  system_table: SystemTable<Boot>,
  devices: Vec<(DeviceID, Device)>
}

impl X8664Platform {
  pub fn new(system_table: SystemTable<Boot>) -> Self {
    Self { 
      system_table,
      devices: Vec::new()
    }
  }

  fn init_interrupts(&self) {
    interrupts::init();
  }
}

impl Platform for X8664Platform {
  type DeviceID = DeviceID;
  type Device = Device;

  fn init(&mut self) {

    // Print out the UEFI revision number
    {
      let rev = self.system_table.uefi_revision();
      let (major, minor) = (rev.major(), rev.minor());

      log::info!("Booted by UEFI {}.{}", major, minor);
    }

    log::info!("Initializing...");

    x86_64::instructions::interrupts::disable();

    log::debug!("- Interrupts");
    self.init_interrupts();

    x86_64::instructions::interrupts::enable();

    self.devices.push((DeviceID::PCKeyboard, Device::PCKeyboard(crate::device::pc_keyboard::PCKeyboard::new())));

    log::info!("Done!");
  }

  fn poll_event(&self) -> Option<PlatformEvent<DeviceID>> {
    event_buffer::poll_event()
  }

  fn sleep(&self) {
    x86_64::instructions::hlt()
  }

  fn device(&mut self, id: &DeviceID) -> Option<&mut Device> {
    for (device_id, device) in self.devices.iter_mut() {
      if id == device_id {
        return Some(device);
      }
    }

    None
  }
}
