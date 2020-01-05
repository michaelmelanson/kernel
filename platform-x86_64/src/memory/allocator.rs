use linked_list_allocator::Heap;
use spin::Mutex;

static FRAME_ALLOCATOR: Mutex<FrameAllocator> = Mutex::new(FrameAllocator {
    heaps: [Heap::empty(); 50]
});

pub fn get() -> &'static Mutex<FrameAllocator> {
    &FRAME_ALLOCATOR
}

pub struct FrameAllocator {
    heaps: [Heap; 50]
}

impl FrameAllocator {
    pub fn add_heap(&mut self, start_address: u64, size: u64) {
        for slot in self.heaps.iter_mut() {
            if slot.size() == 0 {
                let heap = unsafe { Heap::new(start_address as usize, size as usize) };
                *slot = heap;
                return;
            }
        }

        log::warn!("Frame allocator doesn't have any open slots");
    }
}

unsafe impl core::alloc::Alloc for FrameAllocator {
    unsafe fn alloc(&mut self, layout: core::alloc::Layout) -> core::result::Result<core::ptr::NonNull<u8>, core::alloc::AllocErr> { 

        for heap in self.heaps.iter_mut() {
            if let Ok(ptr) = heap.allocate_first_fit(layout) {
                log::debug!("Allocated {} bytes at {:08x}", layout.size(), ptr.as_ptr() as u64);
                return Ok(ptr);
            }
        }

        log::error!("Failed to allocate memory for {:?}", layout);
        return Err(core::alloc::AllocErr);
    }

    unsafe fn dealloc(&mut self, _: core::ptr::NonNull<u8>, _: core::alloc::Layout) { 
        unimplemented!() 
    }
}