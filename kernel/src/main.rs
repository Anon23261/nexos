#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod arch;
mod memory;
mod process;
mod drivers;

/// Kernel entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("NexOS Kernel Starting...");
    
    // Initialize architecture-specific features
    arch::init();
    
    // Initialize memory management
    memory::init();
    
    // Initialize process management
    process::init();
    
    // Initialize device drivers
    drivers::init();
    
    println!("NexOS Kernel Initialized Successfully!");
    
    #[cfg(test)]
    test_main();
    
    loop {
        arch::hlt();
    }
}

/// Panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Kernel panic: {}", info);
    loop {
        arch::hlt();
    }
}

/// Print implementation
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::arch::_print(format_args!($($arg)*)));
}

/// Println implementation
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
