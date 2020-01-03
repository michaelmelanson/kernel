
use super::{
    PlatformEvent,
    device::Device
};

pub trait Platform {
    type DeviceID: core::fmt::Debug + core::cmp::PartialEq;
    type Device: Device;

    fn init(&mut self);
    fn poll_event(&self) -> Option<PlatformEvent<Self::DeviceID>>;
    fn sleep(&self);
    fn device(&mut self, id: &Self::DeviceID) -> Option<&mut Self::Device>;
  }
  