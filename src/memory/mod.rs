pub mod allocator;
pub mod frame_allocator;
pub mod page_table;

use bootloader::bootinfo::BootInfo;
use x86_64::{
    structures::paging::{PageTable, RecursivePageTable},
    VirtAddr,
};

use self::frame_allocator::BumpAllocator;

pub fn init(boot_info: &'static BootInfo) {
    let mut frame_allocator = unsafe {
        BumpAllocator::new(&boot_info.memory_map)
    };

    let phys_mem_offset = VirtAddr::new(0xb8000);  // VGA buffer address for now
    let level_4_table = unsafe { active_level_4_table(phys_mem_offset) };

    let mut recursive_table = RecursivePageTable::new(level_4_table)
        .expect("Failed to create RecursivePageTable");

    allocator::init_heap(&mut recursive_table, &mut frame_allocator)
        .expect("heap initialization failed");
}

unsafe fn active_level_4_table(phys_mem_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();
    let phys = level_4_table_frame.start_address();
    let virt = phys_mem_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr
}
