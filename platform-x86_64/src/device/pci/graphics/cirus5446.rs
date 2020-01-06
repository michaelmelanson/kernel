use crate::{X8664Platform, error::X8664Error, device::DeviceAddress};

#[derive(Clone)]
pub struct Cirus5446 {
    device_address: DeviceAddress,
    framebuffer_address: usize
}

impl Cirus5446 {
    pub fn new(device_address: DeviceAddress) -> Self {
        let framebuffer_address = match device_address {
            DeviceAddress::PCI(ref pci_address) => pci_address.read_dword(0, 0x10) as usize
        };

        Self { device_address, framebuffer_address }
    }

    fn framebuffer(&mut self) -> &mut Framebuffer {
        unsafe { &mut *(self.framebuffer_address as *mut Framebuffer) }
    }
}

impl kernel::Device<X8664Platform> for Cirus5446 {
    fn poll(&mut self) { unimplemented!() }
    fn as_graphics_device(&mut self) -> Option<&mut dyn kernel::GraphicsDevice<X8664Platform>> { Some(self) }
}

impl kernel::GraphicsDevice<X8664Platform> for Cirus5446 {
    fn clear(&mut self) -> Result<(), X8664Error> {
        let framebuffer = self.framebuffer();

        for x in 0..1024 {
            for y in 0..768 {
                framebuffer.pixels[x][y] = BGRA8888Pixel { b: 0, g: 0, r: 0, a: 0 };
            }
        }

        Ok(())
    }
}

#[repr(packed)]
struct BGRA8888Pixel {
    b: u8, 
    g: u8, 
    r: u8, 
    a: u8
}

#[repr(transparent)]
struct Framebuffer {
    pixels: [[BGRA8888Pixel; 768]; 1024] 
}
