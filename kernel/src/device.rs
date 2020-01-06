use alloc::{
    string::String,
    vec::Vec
};
use hashbrown::HashMap;
use crate::Platform;

pub trait Device<P: Platform>: Clone {
    fn poll(&mut self);

    fn as_filesystem(&mut self) -> Option<&mut dyn Filesystem<P>> { None }
    fn as_graphics_device(&mut self) -> Option<&mut dyn GraphicsDevice<P>> { None }
}

pub struct DeviceRegistry<P: Platform> {
    devices: HashMap<P::DeviceID, P::Device>
}

impl <P: Platform> DeviceRegistry<P> {
    pub fn new() -> Self {
        DeviceRegistry {
            devices: HashMap::new()
        }
    }

    pub fn insert(&mut self, id: P::DeviceID, device: P::Device) {
        self.devices.insert(id, device);
    }

    pub fn device(&mut self, id: &P::DeviceID) -> Option<&mut P::Device> {
        self.devices.get_mut(id)
    }

    pub fn filesystem_devices(&mut self) -> Vec<P::DeviceID> {
        let mut devices = Vec::new();

        for (id, device) in self.devices.iter_mut() {
            if let Some(_) = device.as_filesystem() {
                devices.push(*id);
            }
        }

        devices
    }

    pub fn graphics_devices(&mut self) -> Vec<P::DeviceID> {
        let mut devices = Vec::new();

        for (id, device) in self.devices.iter_mut() {
            if let Some(_) = device.as_graphics_device() {
                devices.push(*id);
            }
        }

        devices
    }
}

pub trait Filesystem<P: Platform> {
    fn list(&mut self) -> Result<Vec<String>, P::Error>;
    fn read(&mut self, path: &str) -> Result<Vec<u8>, P::Error>;
}

pub trait GraphicsDevice<P: Platform> {
    fn clear(&mut self) -> Result<(), P::Error>;
}
