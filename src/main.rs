#![feature(lang_items)]
#![feature(asm)]
#![feature(abi_x86_interrupt)]
#![feature(const_fn)]
#![feature(alloc)]
#![feature(allocator_internals)]
#![feature(allocator_api)]
#![no_std] // don't link the Rust standard library
#![cfg_attr(not(test), no_main)] // disable all Rust-level entry points when not testing
#![cfg_attr(test, allow(dead_code, unused_macros))] // disable dead code and unused macro warnings in tests

//#[macro_use]
extern crate alloc;
extern crate bootloader;
extern crate spin;
extern crate volatile;
extern crate x86;
extern crate x86_64;

#[macro_use]
extern crate lazy_static;

#[cfg(test)]
extern crate array_init;

#[cfg(test)]
extern crate std; // use standard library in tests

#[macro_use]
pub mod arch;
pub mod device;
pub mod runtime;

use runtime::allocator::Allocator;

#[global_allocator]
static GLOBAL_ALLOCATOR: Allocator = Allocator::new();

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Kernel starting...");

    use x86::cpuid::CpuId;
    let cpuid = CpuId::new();
    println!("CPU:");

    if let Some(vendor) = cpuid.get_vendor_info() {
        println!("  - Vendor: {}", vendor);
    }

    if let Some(frequency_info) = cpuid.get_processor_frequency_info() {
        println!(
            "  - Base frequency {}",
            frequency_info.processor_base_frequency()
        );
        println!(
            "  - Max frequency {}",
            frequency_info.processor_max_frequency()
        );
    }

    arch::x86_64::pic::remap();
    arch::x86_64::interrupts::install();
    arch::x86_64::pci::pci_probe();
    arch::x86_64::memory::memory_probe();
    arch::x86_64::memory::configure_allocator(&GLOBAL_ALLOCATOR);
    arch::x86_64::paging::init();

    //let list = vec![55, 42];
    //println!("Vector: {:?}", list);

    loop {}
}
