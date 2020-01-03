#![no_std]
#![feature(associated_type_defaults)]

mod device;
mod platform;

pub use crate::{
  device::{Device, DeviceRegistry},
  platform::Platform
};

#[derive(Debug, Clone)]
pub enum PlatformEvent<P: Platform> {
  ClockTicked,
  DeviceConnected(P::DeviceID, P::Device),
  DevicePollable(P::DeviceID)
}

pub struct Kernel<P: Platform> {
  pub platform: P,
  pub device_registry: DeviceRegistry<P>
}

impl <P: Platform> Kernel<P>  {
  pub fn new(platform: P) -> Self {
    Self {
      platform,
      device_registry: DeviceRegistry::new()
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

          PlatformEvent::DeviceConnected(id, device) => {
            self.device_registry.insert(id, device);
          }

          PlatformEvent::DevicePollable(id) => {
            if let Some(device) = self.device_registry.device(&id) {
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
