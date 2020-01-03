use x86_64::structures::idt::*;
use x86_64::instructions::port::Port;
use x2apic::{
  ioapic::{IoApic, IrqFlags, IrqMode},
  lapic::{LocalApic, LocalApicBuilder}
};
use spin::Mutex;

use kernel::{PlatformEvent};
use crate::DeviceID;
use crate::event_buffer::push_event;

lazy_static! {
  static ref IDT: InterruptDescriptorTable = {
    let mut idt = InterruptDescriptorTable::new();

    extern "x86-interrupt" fn divide_error_handler(stack_frame: &mut InterruptStackFrame) {
      panic!("Divide by zero: {:?}", stack_frame);
    }
    idt.divide_error.set_handler_fn(divide_error_handler);

    extern "x86-interrupt" fn page_fault_handler(stack_frame: &mut InterruptStackFrame, error_code: PageFaultErrorCode) {
      panic!("Page fault (error code: {:?}): {:?}", error_code, stack_frame);
    }
    idt.page_fault.set_handler_fn(page_fault_handler);

    extern "x86-interrupt" fn gpf_handler(stack_frame: &mut InterruptStackFrame, error_code: u64) {
      panic!("General protection fault (error code: {}): {:?}", error_code, stack_frame);
    }
    idt.general_protection_fault.set_handler_fn(gpf_handler);

    extern "x86-interrupt" fn double_fault_handler(stack_frame: &mut InterruptStackFrame, error_code: u64) -> ! {
      panic!("Double fault (error code: {}): {:?}", error_code, stack_frame);
    }
    idt.double_fault.set_handler_fn(double_fault_handler);

    extern "x86-interrupt" fn non_maskable_interrupt_handler(stack_frame: &mut InterruptStackFrame) {
      panic!("Non-maskable interrupt: {:?}", stack_frame);
    }
    idt.non_maskable_interrupt.set_handler_fn(non_maskable_interrupt_handler);

    extern "x86-interrupt" fn debug_handler(stack_frame: &mut InterruptStackFrame) {
      panic!("Debug interrupt: {:?}", stack_frame);
    }
    idt.debug.set_handler_fn(debug_handler);

    extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
      log::info!("Breakpoint interrupt: {:?}", stack_frame);
    }
    idt.breakpoint.set_handler_fn(breakpoint_handler);

    extern "x86-interrupt" fn invalid_opcode_handler(stack_frame: &mut InterruptStackFrame) {
      log::info!("Invalid opcode interrupt: {:?}", stack_frame);
    }
    idt.invalid_opcode.set_handler_fn(invalid_opcode_handler);


    extern "x86-interrupt" fn irq_0(stack_frame: &mut InterruptStackFrame) { self::irq_handler(stack_frame, 0x0); }
    idt[0x20].set_handler_fn(irq_0);

    extern "x86-interrupt" fn irq_1(stack_frame: &mut InterruptStackFrame) { self::irq_handler(stack_frame, 0x1); }
    idt[0x21].set_handler_fn(irq_1);

    extern "x86-interrupt" fn irq_2(stack_frame: &mut InterruptStackFrame) { self::irq_handler(stack_frame, 0x2); }
    idt[0x22].set_handler_fn(irq_2);

    extern "x86-interrupt" fn irq_3(stack_frame: &mut InterruptStackFrame) { self::irq_handler(stack_frame, 0x3); }
    idt[0x23].set_handler_fn(irq_3);

    extern "x86-interrupt" fn irq_4(stack_frame: &mut InterruptStackFrame) { self::irq_handler(stack_frame, 0x4); }
    idt[0x24].set_handler_fn(irq_4);

    extern "x86-interrupt" fn irq_5(stack_frame: &mut InterruptStackFrame) { self::irq_handler(stack_frame, 0x5); }
    idt[0x25].set_handler_fn(irq_5);

    extern "x86-interrupt" fn irq_6(stack_frame: &mut InterruptStackFrame) { self::irq_handler(stack_frame, 0x6); }
    idt[0x26].set_handler_fn(irq_6);

    extern "x86-interrupt" fn irq_7(stack_frame: &mut InterruptStackFrame) { self::irq_handler(stack_frame, 0x7); }
    idt[0x27].set_handler_fn(irq_7);

    extern "x86-interrupt" fn irq_8(stack_frame: &mut InterruptStackFrame) { self::irq_handler(stack_frame, 0x8); }
    idt[0x28].set_handler_fn(irq_8);

    extern "x86-interrupt" fn irq_9(stack_frame: &mut InterruptStackFrame) { self::irq_handler(stack_frame, 0x9); }
    idt[0x29].set_handler_fn(irq_9);

    extern "x86-interrupt" fn irq_10(stack_frame: &mut InterruptStackFrame) { self::irq_handler(stack_frame, 0xA); }
    idt[0x2A].set_handler_fn(irq_10);

    extern "x86-interrupt" fn irq_11(stack_frame: &mut InterruptStackFrame) { self::irq_handler(stack_frame, 0xB); }
    idt[0x2B].set_handler_fn(irq_11);

    extern "x86-interrupt" fn irq_12(stack_frame: &mut InterruptStackFrame) { self::irq_handler(stack_frame, 0xC); }
    idt[0x2C].set_handler_fn(irq_12);

    extern "x86-interrupt" fn irq_13(stack_frame: &mut InterruptStackFrame) { self::irq_handler(stack_frame, 0xD); }
    idt[0x2D].set_handler_fn(irq_13);

    extern "x86-interrupt" fn irq_14(stack_frame: &mut InterruptStackFrame) { self::irq_handler(stack_frame, 0xE); }
    idt[0x2E].set_handler_fn(irq_14);

    extern "x86-interrupt" fn irq_15(stack_frame: &mut InterruptStackFrame) { self::irq_handler(stack_frame, 0xF); }
    idt[0x2F].set_handler_fn(irq_15);

    extern "x86-interrupt" fn acpi_gpe(stack_frame: &mut InterruptStackFrame) {
      log::info!("ACPI General Purpose Event: {:?}", stack_frame);
    }
    idt[0x6F].set_handler_fn(acpi_gpe);

    idt
  };

  static ref IOAPIC: Mutex<IoApic> = {
    unsafe {
      let addr = 0xfec00000; // TODO detect this
      let ioapic = IoApic::new(addr);

      Mutex::new(ioapic)
    }
  };

  static ref LAPIC: Mutex<LocalApic> = {  
    let lapic = LocalApicBuilder::new()
      .timer_vector(0)
      .error_vector(2)
      .spurious_vector(3)
      .build()
      .unwrap_or_else(|err| panic!("{}", err));

      Mutex::new(lapic)
  };

}

fn irq_handler(_stack_frame: &mut InterruptStackFrame, irq: u8) {
  match irq {
    0 => push_event(PlatformEvent::ClockTicked),
    1 => push_event(PlatformEvent::DeviceReady(DeviceID::PCKeyboard)),
    _ => {
      log::warn!("Unknown IRQ {}", irq);
    }
  }

  unsafe { LAPIC.lock().end_of_interrupt(); }
}

fn init_local_apic() {
  unsafe {
    LAPIC.lock().enable();
  }
}

fn init_ioapic() {
  unsafe {
    let mut ioapic = IOAPIC.lock();

    ioapic.init(0x20);

    ioapic.enable_irq(
      0,
      0, // CPU(s)
      IrqMode::Fixed,
      IrqFlags::LEVEL_TRIGGERED | IrqFlags::LOW_ACTIVE,
    );

    ioapic.enable_irq(
      1,
      0, // CPU(s)
      IrqMode::Fixed,
      IrqFlags::LOW_ACTIVE,
    );
  }
}

pub fn init() {
  init_ioapic();
  init_local_apic();

  // disable 8259 PICs
  unsafe {
    let mut pic1: Port<u8> = Port::new(0xa1);
    pic1.write(0xff);

    let mut pic2: Port<u8> = Port::new(0x21);
    pic2.write(0xff);
  }
  
  IDT.load();

  log::info!("Initialized interrupts");
}
