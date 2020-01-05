
use super::{
    PlatformEvent,
    device::Device
};

pub trait Platform: Sized {
    type DeviceID: core::marker::Copy + core::fmt::Debug + core::cmp::PartialEq + core::cmp::Eq + core::hash::Hash;
    type Device: Device<Self>;
    type Error: core::fmt::Debug;
    type File;

    fn init(&mut self);
    fn poll_event(&self) -> Option<PlatformEvent<Self>>;
    fn sleep(&self);
}
