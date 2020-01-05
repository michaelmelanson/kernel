mod log_impl;
#[macro_use] mod macros;
mod vga;
mod writer;

pub use log_impl::init;
pub use writer::print;
