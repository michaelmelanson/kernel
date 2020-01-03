#![feature(futures_api)]
#![no_std]

#[macro_use]
extern crate log;

#[derive(Debug, Copy, Clone)]
pub enum Device {
  Keyboard,
  ClockTick
}

#[derive(Debug, Copy, Clone)]
pub enum PlatformEvent {
  DeviceReady(Device)
}

pub trait Platform {
  type KB: Keyboard;

  fn init(&self);
  fn poll_event(&self) -> Option<PlatformEvent>;
  fn sleep(&self);
  fn configure_timer(&self, interval_ms: usize);
  fn keyboard(&self) -> Self::KB;
}

pub struct Kernel<Platform> {
  pub platform: Platform
}

impl <P: Platform> Kernel<P> {
  pub fn new(platform: P) -> Self {
    Self {
      platform
    }
  }

  pub fn start(self) -> ! {
    self.platform.init();

    loop {
      while let Some(event) = self.platform.poll_event() {
       info!("Platform event: {:?}", event);

       match event {
         PlatformEvent::DeviceReady(device) => {
           match device {
             Device::ClockTick => {
               info!("Tick!");
             },
             Device::Keyboard => {
               let keyboard = self.platform.keyboard();
               let c = keyboard.get_char();
               info!("Received key: {:?}", c);
             }
           }
         }
       }
      }

      log::info!("Kernel idle");
      self.platform.sleep();
    }
  }
}

pub trait Keyboard {
  fn get_char(&self) -> Option<char>;
}