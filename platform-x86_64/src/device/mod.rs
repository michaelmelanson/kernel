pub mod pci;
pub mod pc_keyboard;

use crate::X8664Platform;

#[derive(Clone)]
pub enum DeviceAddress {
    PCI(pci::PCIAddress)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DeviceID {
  PCKeyboard,
  Cirus5446
}

#[derive(Clone)]
pub enum Device {
    PCKeyboard(self::pc_keyboard::PCKeyboard),
    Cirus5446(self::pci::graphics::cirus5446::Cirus5446)
}

impl kernel::Device<X8664Platform> for Device {
    fn poll(&mut self) {
        match self {
            Device::PCKeyboard(device) => device.poll(),
            Device::Cirus5446(device) => device.poll(),
        }
    }

    fn as_filesystem(&mut self) -> Option<&mut dyn kernel::Filesystem<X8664Platform>> {
        match self {
            Device::PCKeyboard(device) => device.as_filesystem(),
            Device::Cirus5446(device) => device.as_filesystem(),
        }
    }

    fn as_graphics_device(&mut self) -> Option<&mut dyn kernel::GraphicsDevice<X8664Platform>> {
        match self {
            Device::PCKeyboard(device) => device.as_graphics_device(),
            Device::Cirus5446(device) => device.as_graphics_device(),
        }
    }
}

pub fn discover() {
    pc_keyboard::discover();
    pci::discover();
}