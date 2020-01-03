#![no_std]
#![feature(associated_type_defaults)]

mod device;
mod platform;

pub use crate::{
  device::Device,
  platform::Platform
};

#[derive(Debug, Copy, Clone)]
pub enum PlatformEvent<D> {
  ClockTicked,
  DeviceReady(D)
}

pub struct Kernel<Platform> {
  pub platform: Platform
}

impl <P: Platform> Kernel<P>  {
  pub fn new(platform: P) -> Self {
    Self {
      platform
    }
  }

  pub fn start(mut self) -> ! {
    self.platform.init();

    loop {
      while let Some(event) = self.platform.poll_event() {
        match event {
          PlatformEvent::ClockTicked => {
            log::info!("Tick!");
          },

          PlatformEvent::DeviceReady(id) => {
            if let Some(device) = self.platform.device(&id) {
              device.poll();
            } else {
              log::error!("Unknown device ID: {:?}", id);
            }
          }
        }
      }

      log::debug!("Kernel idle");
      self.platform.sleep();
    }
  }
}
