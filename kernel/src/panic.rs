
/// This function is called on panic.
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let message = info.message();
    let default = &format_args!("No message given");
    let message = message.unwrap_or(default);
    
    if let Some(location) = info.location() {
        log::error!("Panic at {}:{}: {:?}", 
            location.file(),
            location.line(),
            message
        );
    } else {
        log::error!("Panic: {:?}", message);
    }
    loop {}
}

#[alloc_error_handler]
fn alloc_error(_layout: core::alloc::Layout) -> ! {
    unimplemented!()
}