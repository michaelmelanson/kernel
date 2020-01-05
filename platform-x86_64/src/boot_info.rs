
#[derive(Copy, Clone)]
pub struct X8664MemorySegment {
    pub start_address: u64,
    pub length: u64
}

#[derive(Clone)]
pub struct X8664BootInfo {
    pub memory_map: [X8664MemorySegment; 100]
}
