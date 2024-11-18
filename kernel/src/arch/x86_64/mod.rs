use uart_16550::SerialPort;
use spin::Mutex;
use core::fmt;
use lazy_static::lazy_static;

mod gdt;
mod idt;
mod interrupts;

lazy_static! {
    /// Global serial port for debugging
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

/// Initialize x86_64 specific features
pub fn init() {
    // Initialize GDT
    gdt::init();
    
    // Initialize IDT
    idt::init();
    
    // Initialize PIC
    interrupts::init();
    
    println!("x86_64 initialization complete");
}

/// Halt the CPU until the next interrupt
pub fn hlt() {
    x86_64::instructions::hlt();
}

/// Print to serial port
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1.lock().write_fmt(args).expect("Printing to serial failed");
}
