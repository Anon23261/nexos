#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

extern crate alloc;

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

pub mod arch;
pub mod drivers;
pub mod memory;
pub mod process;
pub mod shell;

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // Initialize hardware
    arch::init();
    
    // Initialize memory
    memory::init(boot_info);
    
    // Initialize drivers
    drivers::init();

    #[cfg(test)]
    test_main();

    loop {
        x86_64::instructions::hlt();
    }
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use crate::drivers::vga::{WRITER, Color};
    
    // Change color to red on black for panic message
    WRITER.lock().set_color(Color::Red, Color::Black);
    println!("{}", info);
    
    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
