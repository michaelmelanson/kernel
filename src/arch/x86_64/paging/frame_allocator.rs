use x86_64::structures::paging::{PageSize, PhysFrame};

pub struct FrameAllocator {}

impl<S: PageSize> x86_64::structures::paging::FrameAllocator<S> for FrameAllocator {
    fn alloc(&mut self) -> Option<PhysFrame<S>> {
        None
    }
}
