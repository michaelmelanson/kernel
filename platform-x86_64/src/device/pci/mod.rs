mod discovery;

pub mod graphics;

pub use discovery::discover;

#[derive(Clone)]
pub struct PCIAddress(u8, u8);

impl PCIAddress {
    pub fn new(bus: u8, slot: u8) -> Self { Self(bus, slot) }

    pub fn bus(&self) -> u8 { self.0 }
    pub fn slot(&self) -> u8 { self.1 }

    fn pci_address(&self, function: u8, offset: u8) -> u32 {
        let bus = self.bus() as u32;
        let slot = self.slot() as u32;
        let function = function as u32;
        let offset = offset as u32;

        (bus << 16) 
        | (slot << 11)
        | (function << 8)
        | offset
        | 0x80000000u32
    }

    pub fn read_dword(&self, function: u8, offset: u8) -> u32 {
        use x86_64::instructions::port::Port;

        let mut config_address: Port<u32> = Port::new(0xCF8);
        let mut config_data: Port<u32> = Port::new(0xCFC);

        let address = self.pci_address(function, offset);
        
        unsafe { config_address.write(address); }
        unsafe { config_data.read() }
    }

    pub fn read_word(&self, function: u8, offset: u8) -> u16 {
        let dword = self.read_dword(function, offset & 0xfc);
        let word = (dword >> ((offset&2) * 8) & 0xffff) as u16;

        word
    }
}
