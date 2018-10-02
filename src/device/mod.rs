use arch::x86_64::pci::PCIDeviceAddress;

pub enum DeviceAddress {
    PCIDeviceAddress(PCIDeviceAddress),
}

pub mod network;
