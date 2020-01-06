use super::PCIAddress;
use crate::device::DeviceAddress;

pub fn discover() {
    use crate::{event_buffer, PlatformEvent, device::DeviceID, Device};

    for bus in 0..255 {
        for slot in 0..255 {
            let address = PCIAddress::new(bus, slot);
            let vendor = address.read_word(0, 0);

            if vendor != 0xffff {
                let device = address.read_word(0, 2);

                match (vendor, device) {
                    (0x1234, 0x1111) => {
                        event_buffer::push_event(PlatformEvent::DeviceConnected(
                            DeviceID::Cirus5446, 
                            Device::Cirus5446(super::graphics::cirus5446::Cirus5446::new(DeviceAddress::PCI(address)))
                        ));
                    },

                    (0x1033, 0x194) => {

                    },

                    _ => {
                        log::info!("Unknown PCI device with vendor {:#04x}, device {:#04x}", vendor, device);
                    }
                }
            }
        }
    }
}

