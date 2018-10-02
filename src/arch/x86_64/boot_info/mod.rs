use bootloader::bootinfo::BootInfo;

pub fn get_boot_info() -> BootInfo {
    let boot_info: BootInfo;

    unsafe {
        boot_info = (0xb0071f0000 as *const BootInfo).read();
    }

    return boot_info;
}
