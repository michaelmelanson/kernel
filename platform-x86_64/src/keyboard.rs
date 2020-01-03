use kernel::Keyboard;
use x86_64::instructions::port::Port;

pub struct PCKeyboard;

impl PCKeyboard {
  pub fn new() -> Self { PCKeyboard }
}

impl Keyboard for PCKeyboard {

  fn get_char(&self) -> Option<char> {
    let mut keyboard_controller: Port<u8> = Port::new(0x60);
    let scancode = unsafe { keyboard_controller.read() };
    info!("Keyboard scancode {:#x}", scancode);

    None
  }
}