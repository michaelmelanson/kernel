pub mod pc_keyboard;
//pub mod uefi_filesystem;

use crate::X8664Platform;

#[derive(Clone)]
pub enum Device {
    PCKeyboard(self::pc_keyboard::PCKeyboard),
    //UEFIFilesystem(self::uefi_filesystem::UEFIFilesystem)
}

impl kernel::Device<X8664Platform> for Device {
    fn poll(&mut self) {
        match self {
            Device::PCKeyboard(device) => device.poll(),
           // Device::UEFIFilesystem(device) => device.poll()
        }
    }

    fn as_filesystem(&mut self) -> Option<&mut dyn kernel::Filesystem<X8664Platform>> {
        match self {
            Device::PCKeyboard(device) => device.as_filesystem(),
            //Device::UEFIFilesystem(device) => device.as_filesystem()
        }
    }
}