#![no_std]
#![no_main]
#![feature(abi_efiapi)]

use core::panic::PanicInfo;
use uefi::prelude::*;
use uefi::table::boot::{MemoryDescriptor, MemoryType};
use x86_64::structures::paging::{PageTable, PageTableFlags};

#[entry]
fn efi_main(image: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    
    // Clear the screen
    system_table.stdout().clear().unwrap();
    
    // Print welcome message
    system_table.stdout().write_str("NexOS Bootloader\n").unwrap();
    
    // Get memory map
    let mmap_size = system_table.boot_services().memory_map_size();
    let mmap_buffer = &mut [0u8; 4096];
    let (_key, memory_map) = system_table
        .boot_services()
        .memory_map(mmap_buffer)
        .unwrap();
    
    // Find kernel in filesystem
    // TODO: Implement kernel loading
    
    // Set up initial page tables
    // TODO: Implement page table setup
    
    // Exit boot services
    let (_runtime_table, _memory_map) = system_table
        .exit_boot_services(image, mmap_buffer)
        .unwrap();
    
    // Jump to kernel
    // TODO: Implement kernel jump
    
    Status::SUCCESS
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
