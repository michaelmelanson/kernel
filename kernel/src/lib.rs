#![no_std]
#![feature(associated_type_defaults)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]

extern crate alloc;

mod device;
mod panic;
mod platform;

pub use crate::{
  device::{Device, DeviceRegistry, Filesystem, GraphicsDevice},
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
    log::info!("Kernel starting up");

    self.platform.init();
    self.process_events();

    self.clear_screen().unwrap();

    //self.execute("EFI\\Binaries\\init.efi").unwrap();

    loop {
      self.process_events();

      log::debug!("Kernel idle");
      self.platform.sleep();
    }
  }

  fn process_events(&mut self) {
    while let Some(event) = self.platform.poll_event() {
      match event {
        PlatformEvent::ClockTicked => {
          log::info!("Tick!");
        },

        PlatformEvent::DeviceConnected(id, device) => {
          log::info!("Device {:?} connected", id);
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
  }

  fn clear_screen(&mut self) -> Result<(), P::Error> {
    let devices = self.device_registry.graphics_devices();
    
    for id in devices.iter() {
      log::info!("Found graphics device {:?}", id);
      let device = self.device_registry.device(id).unwrap();
      let device = device.as_graphics_device().unwrap();

      device.clear()?;
    }

    Ok(())
  }

  fn execute(&mut self, path: &str) -> Result<(), P::Error> {

    let filesystem_devices = self.device_registry.filesystem_devices();
    for id in filesystem_devices.iter() {
      let device = self.device_registry.device(id).unwrap();
      let fs = device.as_filesystem().unwrap();
      let contents = fs.read(path)?;

      let binary = goblin::pe::PE::parse(&contents).expect("Failed to parse PE64");
      log::info!("Parsed object: {:#?}", binary);

      for section in binary.sections {
        let name = alloc::string::String::from_utf8(
          section.name.iter()
            .take_while(|c| **c != 0)
            .map(|c| *c)
            .collect::<alloc::vec::Vec<u8>>()
          ).unwrap();
        log::info!(" - Section {:?} ({} bytes to be loaded at {:#016x})", 
          name, section.size_of_raw_data, section.virtual_address);
      }
    }

    Ok(())
  }
}
