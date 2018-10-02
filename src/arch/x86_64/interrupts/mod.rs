use spin::Mutex;
use x86_64::structures::idt::{ExceptionStackFrame, InterruptDescriptorTable, PageFaultErrorCode};

static mut INTERRUPTS: Mutex<InterruptDescriptorTable> =
    Mutex::new(InterruptDescriptorTable::new());

extern "x86-interrupt" fn exception_handler(
    stack_frame: &mut ExceptionStackFrame,
    error_code: u64,
) {
    println!("EXCEPTION:");
    println!("{:#?}", stack_frame);
    println!("Error code: {}", error_code);
    loop {}
}

extern "x86-interrupt" fn trap_handler(stack_frame: &mut ExceptionStackFrame) {
    println!("INTERRUPT: \n{:#?}", stack_frame);
}

type InterruptHandler = extern "x86-interrupt" fn(_: &mut ExceptionStackFrame);
type PageFaultHandler =
    extern "x86-interrupt" fn(_: &mut ExceptionStackFrame, _: PageFaultErrorCode);
type ExceptionHandler = extern "x86-interrupt" fn(_: &mut ExceptionStackFrame, error_code: u64);

pub fn set_page_fault_handler(handler: PageFaultHandler) {
    unsafe {
        INTERRUPTS.lock().page_fault.set_handler_fn(handler);
    }
}

pub fn set_general_protection_fault_handler(handler: ExceptionHandler) {
    unsafe {
        INTERRUPTS
            .lock()
            .general_protection_fault
            .set_handler_fn(handler);
    }
}

pub fn set_double_fault_handler(handler: ExceptionHandler) {
    unsafe {
        INTERRUPTS.lock().double_fault.set_handler_fn(handler);
    }
}

pub fn install() {
    use core::mem::size_of;
    use x86_64::instructions::tables::{lidt, DescriptorTablePointer};

    let mut interrupts = unsafe { INTERRUPTS.lock() };

    extern "x86-interrupt" fn gpf_handler(stack: &mut ExceptionStackFrame, error_code: u64) {
        panic!(
            "General protection fault!\nError code {}, stack: {:?}",
            error_code, stack
        );
    };

    extern "x86-interrupt" fn double_fault_handler(
        stack: &mut ExceptionStackFrame,
        error_code: u64,
    ) {
        panic!(
            "Double fault!\nError code {}, stack: {:?}",
            error_code, stack
        );
    };

    interrupts
        .general_protection_fault
        .set_handler_fn(gpf_handler);
    interrupts.double_fault.set_handler_fn(double_fault_handler);

    extern "x86-interrupt" fn irq_handler(_stack: &mut ExceptionStackFrame) {
        let isr = ::arch::x86_64::pic::get_isr();

        match isr {
            1 => { /* println!("TICK"); */ }
            2 => {
                let scan_code = unsafe { ::x86::io::inb(0x60) };
                println!("Keyboard scan code {}", scan_code);
            }
            irq => {
                panic!("Unknown IRQ {}", irq);
            }
        }

        unsafe {
            x86::io::outb(0x20, 0x20);
        }
    };

    for i in 0x20..0x2F {
        interrupts[i].set_handler_fn(irq_handler);
    }

    unsafe {
        let idt_base = &*interrupts as *const _ as u64;
        let idt_limit = (size_of::<InterruptDescriptorTable>() - 1) as u16;

        let table = DescriptorTablePointer {
            base: idt_base,
            limit: idt_limit,
        };

        println!(
            "Installing IDT: base={:016x?}, limit={:08x?}",
            idt_base, idt_limit
        );

        lidt(&table);
        x86::irq::enable();
    }

    println!("We have interrupts!");
}
