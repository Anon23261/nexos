use x86_64::{
    structures::paging::{
        FrameAllocator, Mapper, Page, PageTable, PageTableFlags, PhysFrame, RecursivePageTable,
        Size4KiB,
    },
    registers::control::Cr3,
    PhysAddr, VirtAddr,
};
use crate::memory::frame_allocator::BumpAllocator;

/// Initialize page tables
pub fn init(level_4_table: &mut PageTable, phys_mem_offset: VirtAddr) -> RecursivePageTable<'_> {
    RecursivePageTable::new(level_4_table).expect("Failed to create RecursivePageTable")
}

/// Map a virtual page to a physical frame
pub fn create_example_mapping(
    page: Page,
    frame: PhysFrame,
    flags: PageTableFlags,
    frame_allocator: &mut BumpAllocator,
) {
    let (level_4_table_frame, _) = Cr3::read();
    let level_4_table_addr = level_4_table_frame.start_address();
    
    let recursive_page_table = unsafe {
        let level_4_table_ptr = 0xffff_ffff_ffff_f000 as *mut PageTable;
        &mut *level_4_table_ptr
    };
    
    let mut recursive_page_table = unsafe {
        RecursivePageTable::new(recursive_page_table).unwrap()
    };
    
    unsafe {
        recursive_page_table
            .map_to(page, frame, flags, frame_allocator)
            .unwrap()
            .flush();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn test_page_table_init() {
        let (level_4_table_frame, _) = Cr3::read();
        let level_4_table_addr = level_4_table_frame.start_address();
        let mut page_table = unsafe { &mut *(level_4_table_addr.as_u64() as *mut PageTable) };
        let phys_mem_offset = VirtAddr::from_ptr(0x1000 as *const u8);
        init(&mut page_table, phys_mem_offset);
        // Add more specific tests here
    }
}
