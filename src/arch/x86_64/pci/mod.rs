pub struct PCIDeviceAddress {
    bus: u8,
    slot: u8,
}

impl PCIDeviceAddress {
    fn new(bus: u8, slot: u8) -> PCIDeviceAddress {
        PCIDeviceAddress {
            bus: bus,
            slot: slot,
        }
    }

    fn read(&self, function: u8, offset: u8) -> u16 {
        use x86::io::{inl, outl};

        let address: u32 = ((self.bus as u32) << 16)
            | ((self.slot as u32) << 11)
            | ((function as u32) << 8)
            | ((offset as u32) & 0xfc)
            | 0x80000000;

        unsafe {
            outl(0xcf8, address);
            (inl(0xcfc) >> ((offset & 2) * 8) & 0xffff) as u16
        }
    }

    fn get_info(&self) -> Option<PCIDevice> {
        let vendor_id = self.read(0, 0);
        if vendor_id == 0xffff {
            return None;
        }

        let device_id = self.read(0, 2);

        let value = self.read(0, 10);
        let class = (value & 0xff) as u8;
        let subclass = (value >> 8) as u8;

        Some(PCIDevice {
            vendor_id: vendor_id,
            device_id: device_id,
            class: class,
            subclass: subclass,
        })
    }
}

#[derive(Debug)]
struct PCIDevice {
    vendor_id: u16,
    device_id: u16,

    class: u8,
    subclass: u8,
}

pub fn pci_probe() {
    println!("Probing PCI:");

    for bus in 0..255 {
        for slot in 0..31 {
            let address = PCIDeviceAddress::new(bus, slot);
            if let Some(info) = address.get_info() {
                println!("  - Bus {}, Slot {}: {:x?}", bus, slot, info);
            }
        }
    }

    println!("PCI probe finished");
}
