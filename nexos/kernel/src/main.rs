#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
mod memory;
mod process;
mod drivers;
mod arch;

/// Entry point for the kernel
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
        // Main kernel loop
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

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
