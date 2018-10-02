use arch::x86_64::boot_info::get_boot_info;
use bootloader::bootinfo::MemoryRegion;
use x86_64::registers::control::Cr3;
use x86_64::structures::idt::{ExceptionStackFrame, PageFaultErrorCode};
use x86_64::structures::paging::{
    Mapper, Page, PageTable, PageTableFlags, PhysFrame, RecursivePageTable, Size2MiB,
};
use x86_64::{PhysAddr, VirtAddr};

mod frame_allocator;
use self::frame_allocator::FrameAllocator;

lazy_static! {
    static ref FRAME_ALLOCATOR: spin::Mutex<FrameAllocator> = spin::Mutex::new(FrameAllocator {});
}

fn get_page_table() -> *mut PageTable {
    let (page_table_ptr, flags) = Cr3::read();
    println!("CR3 flags: {:?}", flags);

    let page = Page::<Size2MiB>::containing_address(VirtAddr::new(
        page_table_ptr.start_address().as_u64(),
    ));
    println!(
        "Page table pointer: {:?} (p2: {}, p3: {}, p4: {})",
        page,
        page.p2_index(),
        page.p3_index(),
        page.p4_index()
    );

    let page_table = unsafe { &mut *(page_table_ptr.start_address().as_u64() as *mut PageTable) };

    page_table
}

fn memory_region_for_address(address: VirtAddr) -> Option<MemoryRegion> {
    let boot_info = get_boot_info();

    for r in boot_info.memory_map.iter() {
        if address.as_u64() >= r.range.start_addr() && address.as_u64() < r.range.end_addr() {
            return Some(*r);
        }
    }

    None
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: &mut ExceptionStackFrame,
    error_code: PageFaultErrorCode,
) {
    println!("PAGE ZFAULT!");

    let address: VirtAddr = unsafe { VirtAddr::new(x86::controlregs::cr2() as u64) };
    println!("Page fault for address: {:#016x}", address.as_u64());

    let invalid_page_fault = || {
        panic!(
            "Invalid page fault for {:#016x}.\nStack frame: {:?}.\nError code: {:?}",
            address.as_u64(),
            stack_frame,
            error_code
        );
    };

    let region: MemoryRegion =
        memory_region_for_address(address).unwrap_or_else(invalid_page_fault);
    println!("Address is in region: {:?}", region);

    let page = Page::<Size2MiB>::containing_address(address);
    println!("Page: {:?}", page);

    let mut page_table = get_page_table();

    let phys_addr = PhysAddr::new(address.as_u64());
    let phys_frame = PhysFrame::<Size2MiB>::containing_address(phys_addr);

    println!("TODO identity map {:#016x}", phys_frame.start_address());
    loop {
        unsafe {
            x86::halt();
        }
    }

    //page_table.identity_map(phys_frame, PageTableFlags::PRESENT, &mut *FRAME_ALLOCATOR.lock()).unwrap().flush();

    //println!("All done page fault!");
}

pub fn init() {
    ::arch::x86_64::interrupts::set_page_fault_handler(page_fault_handler);

    let page_table: *mut PageTable = get_page_table();

    //println!("Page table:\n{:#?}", unsafe { &(*page_table)[0] });
}
