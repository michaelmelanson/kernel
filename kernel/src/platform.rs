
use super::{
    PlatformEvent,
    device::Device
};

pub trait Platform: Sized {
    type DeviceID: core::fmt::Debug + core::cmp::PartialEq + core::cmp::Eq + core::hash::Hash;
    type Device: Device;

    fn init(&mut self);
    fn poll_event(&self) -> Option<PlatformEvent<Self>>;
    fn sleep(&self);
  }
  