pub mod keyboard;
pub mod vga;

use crate::println;
use x86_64::instructions::interrupts;

pub fn init() {
    vga::init();
    keyboard::init();
    
    // Enable interrupts after all drivers are initialized
    interrupts::enable();
    println!("Drivers initialized");
}
