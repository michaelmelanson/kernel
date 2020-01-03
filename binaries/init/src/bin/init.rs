#![no_std]
#![no_main]
extern crate libuser;

#[no_mangle]
pub extern "win64" fn uefi_start() -> ! {
    panic!("Init loaded");
}
