use spin::Mutex;

use super::vga;

lazy_static! {
    static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        colour_code: vga::ColourCode::new(vga::Colour::Yellow, vga::Colour::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut vga::Buffer) },
    });
}

pub struct Writer {
    pub column_position: usize,
    pub colour_code: vga::ColourCode,
    pub buffer: &'static mut vga::Buffer
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),

            byte => {
                if self.column_position >= vga::BUFFER_WIDTH {
                    self.new_line()
                }

                let row = vga::BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let colour_code = self.colour_code;
                self.buffer.chars[row][col] = vga::Char {
                    ascii_character: byte,
                    colour_code
                };
                self.column_position += 1;
            }
        }
    }

    pub fn new_line(&mut self) { 
        // unimplemented!()
        self.column_position = 0;
    }
}


impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::result::Result<(), core::fmt::Error> { 
        write_to_serial_out(s);

        for b in s.bytes() {
            match b {
                0x20..=0x7e => self.write_byte(b),
                _ => self.write_byte(0x7e)
            }
        }
        
        Ok(())
    }
}

pub fn print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

fn write_to_serial_out(s: &str) {
    let mut stdout = x86_64::instructions::port::Port::new(0x3f8);

    for c in s.bytes() {
        unsafe { stdout.write(c as u8); }
    }
}