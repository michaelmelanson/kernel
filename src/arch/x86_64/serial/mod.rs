use spin::Mutex;
use x86::io::{inb, outb};

struct SerialPort {
    address: u16,
}

impl SerialPort {
    fn new(address: u16) -> SerialPort {
        unsafe {
            outb(address + 1, 0x00); // Disable all interrupts
            outb(address + 3, 0x80); // Enable DLAB (set baud rate divisor)
            outb(address + 0, 0x03); // Set divisor to 3 (lo byte) 38400 baud
            outb(address + 1, 0x00); //                  (hi byte)
            outb(address + 3, 0x03); // 8 bits, no parity, one stop bit
            outb(address + 2, 0xC7); // Enable FIFO, clear them, with 14-byte threshold
            outb(address + 4, 0x0B); // IRQs enabled, RTS/DSR set
        }
        SerialPort { address: address }
    }

    /*
      unsafe fn can_read(&self) -> bool {
        (inb(self.address + 5) & 1) != 0
      }
    
      unsafe fn read(&self) -> u8 {
        while !self.can_read() {}
        inb(self.address)
      }
    */

    unsafe fn can_write(&self) -> bool {
        (inb(self.address + 5) & 0x20) != 0
    }

    unsafe fn write(&self, c: u8) {
        while !self.can_write() {}
        outb(self.address, c);
    }
}

lazy_static! {
    static ref COM1: Mutex<SerialPort> = Mutex::new(SerialPort::new(0x3f8));
}

pub fn write_com1(byte: u8) {
    unsafe {
        COM1.lock().write(byte);
    }
}
