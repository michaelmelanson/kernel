#![feature(abi_x86_interrupt)]
#![feature(core_intrinsics)]
#![no_std]

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

mod interrupts;
mod keyboard;

use kernel::{Platform, PlatformEvent};
use ringbuffer::RingBuffer;

use keyboard::PCKeyboard;

lazy_static! {
  static ref EVENT_BUFFER: RingBuffer<PlatformEvent> = {
    RingBuffer::new_with_capacity(1000)
  };
}

pub(crate) fn push_event(event: PlatformEvent) {
  EVENT_BUFFER.push(event);
}

pub struct X8664Platform;

impl X8664Platform {
  pub fn new() -> Self {
    Self
  }

  fn init_interrupts(&self) {
    interrupts::init();
  }
}

impl Platform for X8664Platform {
  type KB = PCKeyboard;

  fn init(&self) {
    info!("Initializing...");

    x86_64::instructions::interrupts::disable();

    debug!("- Interrupts");
    self.init_interrupts();

    x86_64::instructions::interrupts::enable();

    info!("Done!");
  }

  fn poll_event(&self) -> Option<PlatformEvent> {
    EVENT_BUFFER.poll()
  }

  fn sleep(&self) {
    x86_64::instructions::hlt()
  }

  fn configure_timer(&self, interval: usize) {

  }

  fn keyboard(&self) -> PCKeyboard {
    PCKeyboard::new()
  }
}
