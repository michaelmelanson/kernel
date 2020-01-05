use x86_64::instructions::port::Port;

use crate::X8664Platform;

#[derive(Clone)]
pub struct PCKeyboard;

impl PCKeyboard {
  pub fn new() -> Self { PCKeyboard }
}

impl kernel::Device<X8664Platform> for PCKeyboard {
  fn poll(&mut self) {
    let mut keyboard_controller: Port<u8> = Port::new(0x60);
    let scancode = unsafe { keyboard_controller.read() };
    log::info!("Keyboard scancode {:#x}", scancode);
  }
}