pub mod pc_keyboard;

pub enum Device {
    PCKeyboard(self::pc_keyboard::PCKeyboard)
}

impl kernel::Device for Device {
    fn poll(&mut self) {
        match self {
            Device::PCKeyboard(keyboard) => keyboard.poll()
        }
    }
}