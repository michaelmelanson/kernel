use hashbrown::HashMap;
use crate::Platform;

pub trait Device: Clone {
    fn poll(&mut self);
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
}