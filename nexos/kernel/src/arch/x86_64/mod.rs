use x86_64::{
    instructions::{interrupts, port::Port},
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame},
};

mod gdt;
mod idt;
mod interrupts;

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

/// Initialize architecture-specific features
pub fn init() {
    // Initialize GDT
    gdt::init();
    
    // Initialize IDT
    unsafe {
        IDT.breakpoint.set_handler_fn(breakpoint_handler);
        IDT.double_fault.set_handler_fn(double_fault_handler);
        IDT.load();
    }
    
    // Initialize interrupt controller
    interrupts::init();
    
    println!("Architecture initialization complete");
}

/// Halt the CPU until the next interrupt
pub fn hlt() {
    x86_64::instructions::hlt();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn test_breakpoint_exception() {
        x86_64::instructions::interrupts::int3();
    }
}
