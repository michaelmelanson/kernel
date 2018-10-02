use core::alloc::{Alloc, GlobalAlloc, Layout};
use core::ptr::NonNull;
use spin::Mutex;

mod bump_allocator;

use self::bump_allocator::BumpAllocator;

#[default_lib_allocator]
pub struct Allocator {
    allocator: Mutex<BumpAllocator>,
}

impl Allocator {
    pub const fn new() -> Allocator {
        Allocator {
            allocator: Mutex::new(BumpAllocator::new()),
        }
    }

    pub fn add_memory_region(&self, start: usize, end: usize) {
        self.allocator.lock().set_memory_region(start, end);
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut allocator = self.allocator.lock();

        match allocator.alloc(layout) {
            Ok(addr) => addr.as_ptr(),
            Err(err) => panic!("Allocation failed: {:?}", err),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let mut allocator = self.allocator.lock();

        match NonNull::new(ptr) {
            Some(ptr) => allocator.dealloc(ptr, layout),
            None => panic!("Attempt to dealloc null pointer"),
        }
    }
}

#[cfg(not(test))]
#[lang = "oom"]
#[no_mangle]
pub extern "C" fn rust_oom(_: core::alloc::Layout) -> ! {
    panic!("Out of memory!");
}
