use super::allocator;

#[global_allocator]
static GLOBAL_ALLOC: GlobalAllocator = GlobalAllocator;
struct GlobalAllocator;

unsafe impl alloc::alloc::GlobalAlloc for GlobalAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        use core::alloc::Alloc;
        allocator::get().lock().alloc(layout)
            .expect("Failed to allocate memory")
            .as_ptr()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) { 
        use core::alloc::Alloc;
        let ptr = core::ptr::NonNull::new(ptr).expect("Tried to dealloc null pointer");
        allocator::get().lock().dealloc(ptr, layout)
    }
}
