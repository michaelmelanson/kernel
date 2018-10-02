use core::alloc::Alloc;
use core::alloc::AllocErr;
use core::alloc::Layout;
use core::ptr::NonNull;

pub struct BumpAllocator {
    start: usize,
    end: usize,
    next: usize,
}

impl BumpAllocator {
    pub const fn new() -> BumpAllocator {
        BumpAllocator {
            start: 0,
            end: 0,
            next: 0,
        }
    }

    pub fn set_memory_region(&mut self, start: usize, end: usize) {
        self.start = start;
        self.end = end;
        self.next = start;
    }
}

unsafe impl Alloc for BumpAllocator {
    unsafe fn alloc(&mut self, layout: Layout) -> Result<NonNull<u8>, AllocErr> {
        let addr = self.next;
        self.next += layout.size();

        if self.next <= self.end {
            println!("Allocated {} bytes at {:#016x}.", layout.size(), addr);
            Ok(NonNull::new(addr as *mut u8).unwrap())
        } else {
            Err(AllocErr)
        }
    }

    unsafe fn dealloc(&mut self, ptr: NonNull<u8>, _layout: Layout) {
        let address: usize = ptr.as_ptr() as usize;
        println!("Dealloc called for {:#016x}.", address);
    }
}
