#![no_std]
#![feature(asm,lang_items)]

extern crate x86_64;

use core::panic::PanicInfo;

// These functions are used by the compiler, but not
// for a bare-bones hello world. These are normally
// provided by libstd.
#[lang = "eh_personality"]
#[no_mangle]
pub extern fn rust_eh_personality() {
}

// This function may be needed based on the compilation target.
#[lang = "eh_unwind_resume"]
#[no_mangle]
pub extern fn rust_eh_unwind_resume() {
}

#[lang = "panic_impl"]
#[no_mangle]
pub extern fn rust_begin_panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe {
            asm!("int $0" :: "i" (0x55) :: "volatile");
        }
    }
    
}