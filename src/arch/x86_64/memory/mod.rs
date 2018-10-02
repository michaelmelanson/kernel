use arch::x86_64::boot_info::get_boot_info;
use bootloader::bootinfo::MemoryRegionType;

use runtime::allocator::Allocator;

pub fn memory_probe() {
    let boot_info = get_boot_info();

    println!("All memory regions:");
    for region in boot_info.memory_map.iter() {
        println!("  - {:?}", region);
    }

    println!("Usable memory regions:");
    for region in boot_info.memory_map.iter() {
        if region.region_type == MemoryRegionType::Usable {
            let start = region.range.start_addr();
            let end = region.range.end_addr();
            let size = end - start;

            println!(
                "  - {:#016x} - {:#016x} ({}MB)",
                start,
                end,
                size / (1024 * 1024)
            );
        }
    }
}

pub fn configure_allocator(allocator: &Allocator) {
    let boot_info = get_boot_info();

    for region in boot_info.memory_map.iter() {
        if region.region_type == MemoryRegionType::Usable {
            allocator.add_memory_region(
                region.range.start_addr() as usize,
                region.range.end_addr() as usize,
            );
        }
    }
}
