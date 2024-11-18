pub mod gdt;
pub mod interrupts;

pub fn init() {
    gdt::init();
    interrupts::init();
    unsafe { interrupts::PICS.lock().initialize() };
}

pub fn hlt() {
    x86_64::instructions::hlt();
}
